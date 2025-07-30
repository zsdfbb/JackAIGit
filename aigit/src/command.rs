use clap::Parser;
use log::{debug, info};
use std::error::Error;
use std::process::{Child, Command, Stdio};
use std::vec;

use crate::config::{G_AI_API_KEY, G_AI_MODEL, G_AI_PLATFORM};
use crate::api::{self, get_chat, get_platform_list, ChatMessage, ChatRequest};
use crate::ollama::{self};

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    /// Show supported platforms
    #[arg(short, long)]
    platforms: bool,
    // subcommand
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Show the diff between the working tree and the index
    Diff {
        /// specify the commit index such as HEAD^
        index: Option<String>,
        /// explain the diff between the working tree and the index
        #[arg(short, long)]
        explain: bool,
    },
    /// Commit the current changes
    Commit {
        /// specify the commit message
        #[arg(short, long)]
        message: Option<String>,
        /// generate commit message
        #[arg(short, long)]
        explain: bool,
        /// specify the git commit message template
        #[arg(short, long)]
        template: Option<String>,
    },
    /// List all commits
    List {
        /// number of commits
        #[arg(short, long)]
        number: Option<u32>,
        /// explain selected commit
        #[arg(short, long)]
        explain: bool,
    },
    /// Show commit details
    Show {
        /// commit hash
        #[arg(short, long)]
        hash: String,
        /// explain selected commit
        #[arg(short, long)]
        explain: bool,
    },

}

fn prompt_diff(diff_content: String) -> Vec<ChatMessage> {
    let mut diff_msgs: Vec<ChatMessage> = vec![
        ChatMessage {
            role: "system".to_string(),
            content: "你是一个资深软件工程师，擅长解析Git补丁。请分析以下Git Patch内容，用清晰的结构解释修改内容：

### 输入要求：
1. 用户会提供一段Git Patch文本（格式为`git diff`输出）
2. 你需要提取以下关键信息：
   - 修改的文件路径
   - 每个文件的变更类型（添加/删除/修改/重命名）
   - 代码变更的核心逻辑（用自然语言描述）
   - 特别注意`@@`行号范围标识的代码块
   - 如果存在冲突标记（如`<<<<<<<`），需单独说明

### 输出规范：
用以下Markdown格式回复：
```markdown
## 分析报告
**总览**:
- 修改文件数：X  
- 主要变更类型：[功能新增/缺陷修复/重构/配置调整等]

### 文件分析：
1. **文件路径**：`src/example.py`  
   - **变更类型**：修改  
   - **行号范围**：@@ -15,6 +15,8 @@ (表示原文件第15行起6行 → 新版本第15行起8行)  
   - **变更描述**：  
     - 在`calculate()`函数中添加了参数校验逻辑  
     - 修复了除零错误（新增第18-19行）  
     - 删除过时的日志输出（原第22行）

2. **文件路径**：`config/env.yaml`  
   - **变更类型**：新增  
   - **关键变更**：添加了数据库连接池配置参数

[按此格式继续其他文件...]

### 注意：
- 用`>` 符号引用关键代码片段（不超过3行）
- 发现冲突标记时用⚠️警告
- 不要猜测未出现在Patch中的上下文".to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: "你好，请使用中文解释下面的代码修改：\n".to_string(),
        },
    ];

    diff_msgs[1].content.push_str(diff_content.as_str());
    diff_msgs
}

fn prompt_create_commit_msg(commit_content: String) -> Vec<ChatMessage> {
    let mut commit_msgs: Vec<ChatMessage> = vec![
        ChatMessage {
            role: "system".to_string(),
            content: "You are an expert in conventional commits. Generate a commit message strictly following this format:

<type>(<scope>): <subject>
<body>
<footer>

Rules:
1. **Type** (mandatory): Choose exactly one from:
   - feat: new feature
   - fix: bug fix
   - docs: documentation changes
   - style: formatting changes (whitespace, formatting, etc.)
   - refactor: code restructuring (non-breaking changes)
   - test: test additions/modifications
   - chore: build/auxiliary tool updates

2. **Scope** (optional): Module/component affected (e.g., 'login', 'database'). Omit if irrelevant.

3. **Subject** (mandatory):
   - Imperative tense ('Add' not 'Added')
   - ≤50 characters
   - No ending punctuation
   - Summarize key change

4. **Body** (optional):
   - Detailed explanation s(72-character wrap)
   - Answer 'why?' not 'what?'
   - Use bullet points if needed

5. **Footer** (optional):
   - Reference issues (e.g., 'Closes #123')
   - BREAKING CHANGE notices if applicable

Analyze this git diff and generate the commit message accordingly. Output ONLY the commit message with no additional text.".to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: "This is Git diff:\n".to_string(),
        },
    ];

    commit_msgs[1].content.push_str(commit_content.as_str());

    commit_msgs
}


fn prompt_show(commit_content: String) -> Vec<ChatMessage> {
    let mut tmp_prompt: Vec<ChatMessage> = vec![
        ChatMessage {
            role: "system".to_string(),
            content: "".to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: "This is Git diff:\n".to_string(),
        },
    ];

    tmp_prompt[1].content.push_str(commit_content.as_str());

    tmp_prompt
}

fn get_git_res(child: Child) -> Result<String, Box<dyn Error>> {
    let output = child.wait_with_output()?;

    match output.status.code() {
        Some(0) => {
            return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
        }
        Some(128) => {
            return Err("Not a git repository (or any parent directory)".into());
        }
        _ => {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
    }
}

fn get_git_diff(index: String) -> Result<String, Box<dyn Error>> {
    let args = [
        "diff",       // 显示当前修改
        "--no-color", // 禁用外部差异工具
        "-U3",        // 显示3行上下文
        index.as_str(),
    ];

    let child = Command::new("git")
        .args(args)
        .stdout(Stdio::piped()) // 捕获标准输出
        .stderr(Stdio::piped()) // 捕获错误输出
        .spawn()?; // 异步启动

    return get_git_res(child);
}

fn get_git_show(index: String) -> Result<String, Box<dyn Error>> {
    let args = [
        "show",       // 显示指定index
        "--no-color", // 禁用外部差异工具
        "-U3",        // 显示3行上下文
        index.as_str(),
    ];

    let child = Command::new("git")
        .args(args)
        .stdout(Stdio::piped()) // 捕获标准输出
        .stderr(Stdio::piped()) // 捕获错误输出
        .spawn()?; // 异步启动

    return get_git_res(child);
}

fn handle_diff(index: String, explain: bool) -> Result<(), Box<dyn Error>> {
    // 输出 diff 内容
    let diff_content = get_git_diff(index)?;
    // println!("{}", diff_content);

    if explain {
        debug!("Explaining...");
        let chat: api::ChatFn = get_chat(G_AI_PLATFORM.clone());
        let _ = chat(G_AI_MODEL.clone(), G_AI_API_KEY.clone(), prompt_diff(diff_content));
    }

    Ok(())
}

fn handle_show(index: String, explain: bool) -> Result<(), Box<dyn Error>> {
    // 输出 diff 内容
    let show_content = get_git_show(index)?;
    println!("{}", show_content);

    if explain {
        debug!("Explaining...");
        ollama::chat(G_AI_MODEL.clone(), G_AI_API_KEY.clone(), prompt_show(show_content));
    }

    Ok(())
}

pub fn handle() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if cli.platforms {
        let pl = get_platform_list();
        println!("Supported Platforms:");
        pl.iter().for_each(|item| println!("{}", item));
    }

    match cli.command {
        Some(Commands::Diff { index, explain }) => {
            handle_diff(index.unwrap_or("HEAD^".to_string()), explain)?;
        }
        Some(Commands::Commit {
            message,
            explain,
            template,
        }) => {}
        Some(Commands::List { number, explain }) => {}
            Some(Commands::Show { hash, explain }) => {
            handle_show(hash, explain)?;
        }
        _ => {}
    }

    Ok(())
}

/*
 * ===========================================================
 * test code
 */
#[cfg(feature = "test")]
fn test_options() {
    println!("command: test options");
}

#[cfg(feature = "test")]
fn test_parse_subcommand() {
    println!("command: test subcommand");

    let cli = Cli::parse();
    match cli.command {
        Commands::Diff { index, explain } => {
            println!("Diff index: {}, explain: {:?}", index, explain);
        }
        Commands::Commit { message, explain } => {
            println!(
                "Commit specify message: {:?}, explain: {:?}",
                message, explain
            );
        }
        Commands::List { number, explain } => {
            println!("List num: {:?}, explain: {:?}", number, explain);
        }
    }
}

#[cfg(feature = "test")]
pub fn test() {
    test_options();
    test_parse_subcommand();
}
