use clap::Parser;
#[allow(unused_imports)]
use log::{debug, error, info};
use std::error::Error;
use std::process::{Child, Command, Stdio};
use std::vec;

use crate::api::common::{ChatFn, ChatMessage, get_chat, get_platform_list};
use crate::config::{G_AI_API_KEY, G_AI_MODEL, G_AI_PLATFORM};

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
        /// specify the path\file\commit
        index: Option<String>,
        /// explain the diff between the working tree and the index
        #[arg(short, long)]
        explain: bool,
    },
    /// Commit the current changes
    Commit {
        /// generate commit message
        #[arg(short, long)]
        explain: bool,
        /// sign the commit
        #[arg(short, long)]
        signoff: bool,
        /// Directly use AI-generated commit message
        #[arg(short, long)]
        direct: bool,
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
        hash: Option<String>,
        /// explain selected commit
        #[arg(short, long)]
        explain: bool,
    },
}

fn prompt_diff(diff_content: String) -> Vec<ChatMessage> {
    let mut diff_msgs: Vec<ChatMessage> = vec![
        ChatMessage {
            role: "system".to_string(),
            content: 
"You are a senior software engineer skilled in parsing Git patches. Please analyze the following Git patch content and explain the changes in a clear, structured manner:

### Input Requirements:
    1. The user will provide a Git patch text (in `git diff` output format).  
    2. You must extract the following key information:  
    - Modified file paths  
    - Change type for each file (Added/Deleted/Modified/Renamed)  
    - Core logic of code changes (described in natural language)  
    - Pay special attention to code blocks marked by `@@` line range indicators  
    - If conflict markers (e.g., `<<<<<<<`) exist, highlight them separately  

 ### Output Specification:
    Reply using the following Markdown format:  
    ````markdown  
    ## Analysis Report  
    **Overview**:  
    - Modified files: X  
    - Primary change type: [Feature addition/Bug fix/Refactoring/Configuration adjustment/etc.]  

### File Analysis:  
    1. **File path**: `src/example.py`  
    - **Change type**: Modified  
    - **Line range**: @@ -15,6 +15,8 @@ (Original: 6 lines from line 15 → New: 8 lines from line 15)  
    - **Change description**:  
        - Added parameter validation logic in `calculate()` function  
        - Fixed division-by-zero error (new lines 18-19)  
        - Removed deprecated log output (original line 22)  

    2. **File path**: `config/env.yaml`  
    - **Change type**: Added  
    - **Key changes**: Added database connection pool parameters  

    [Continue for other files in this format...]  

### Notes:  
    - Use `>` to quote critical code snippets (≤ 3 lines)  
    - Use ⚠️ warning when conflict markers are detected  
    - Do not speculate about context not present in the patch"
            .to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: "Hello, please explain the code modification below. \n".to_string(),
        },
    ];

    diff_msgs[1].content.push_str(diff_content.as_str());
    diff_msgs
}

fn prompt_create_commit_msg(commit_content: String) -> Vec<ChatMessage> {
    let mut commit_msgs: Vec<ChatMessage> = vec![
        ChatMessage {
            role: "system".to_string(),
            content: 
"You are an expert in conventional commits. Generate a commit message strictly following this format:

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

Analyze this git diff and generate the commit message accordingly. 
Output ONLY the commit message with no additional text."
            .to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: 
"Please help me to generate commit message. 
Please populate the content within the <body> section according to the file granularity.
The following is Git patch's description:\n"
.to_string(),
        },
    ];

    commit_msgs[1].content.push_str(commit_content.as_str());

    commit_msgs
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

fn git_diff(index: String, only_staged: bool) -> Result<String, Box<dyn Error>> {
    let mut args = vec![
        "diff",       // 显示当前修改
        "--no-color", // 禁用外部差异工具
        "-U3",        // 显示3行上下文
    ];

    if only_staged {
        args.push("--staged");
    }
    args.push(&index.as_str());

    let child = Command::new("git")
        .args(args)
        .stdout(Stdio::piped()) // 捕获标准输出
        .stderr(Stdio::piped()) // 捕获错误输出
        .spawn()?; // 异步启动

    return get_git_res(child);
}

fn handle_diff(index: String, explain: bool) -> Result<(), Box<dyn Error>> {
    // 输出 diff 内容
    let diff_content = git_diff(index, false)?;
    println!("============================================================================");
    println!("Git Diff Content");
    println!("============================================================================");
    println!("{}", diff_content);

    if explain {
        println!("============================================================================");
        println!("Explaining...\n");
        let chat: ChatFn = get_chat(G_AI_PLATFORM.clone());
        let diff_explain = chat(
            G_AI_MODEL.clone(),
            G_AI_API_KEY.clone(),
            prompt_diff(diff_content),
        )?;
        println!("{}", diff_explain);
    }

    Ok(())
}

fn git_show(hash: String) -> Result<String, Box<dyn Error>> {
    let args = [
        "show",       // 显示指定index
        "--no-color", // 禁用外部差异工具
        "-U3",        // 显示3行上下文
        hash.as_str(),
    ];

    let child = Command::new("git")
        .args(args)
        .stdout(Stdio::piped()) // 捕获标准输出
        .stderr(Stdio::piped()) // 捕获错误输出
        .spawn()?; // 异步启动

    return get_git_res(child);
}

fn handle_show(hash: String, explain: bool) -> Result<(), Box<dyn Error>> {
    // 输出 diff 内容
    let show_content = git_show(hash)?;
    println!("============================================================================");
    println!("Git Show Content");
    println!("============================================================================");
    println!("{}", show_content);

    if explain {
        println!("============================================================================");
        println!("Explaining...\n");
        let chat: ChatFn = get_chat(G_AI_PLATFORM.clone());
        let show_explain = chat(
            G_AI_MODEL.clone(),
            G_AI_API_KEY.clone(),
            prompt_diff(show_content),
        )?;
        println!("{}", show_explain);
    }

    Ok(())
}

fn git_commit(signoff: bool, directly: bool, message: String) -> Result<String, Box<dyn Error>> {
    let mut v_args = vec!["commit"];
    
    if signoff {
        v_args.push("--signoff");
    }
    if !directly {
        v_args.push("--edit");
    }

    v_args.push("-m");
    v_args.push(message.as_str());

    let status = Command::new("git")
        .args(v_args)
        .status()?; // 前台启动

    if status.success() {
        Ok("Commit successful".to_string())
    } else {
        Err("Commit failed".into())
    }
}

fn handle_commit(explain: bool, signoff: bool, directly: bool) -> Result<(), Box<dyn Error>> {
    let diff_content = git_diff("HEAD".to_string(), true)?;
    println!("============================================================================");
    println!("Git commit Content");
    println!("============================================================================");
    println!("{}", diff_content);

    let mut cm_msg = String::from("# Please edit commit message");
    if explain {
        println!("============================================================================");
        println!("Explaining...");
        let chat: ChatFn = get_chat(G_AI_PLATFORM.clone());
        let diff_explain = chat(
            G_AI_MODEL.clone(),
            G_AI_API_KEY.clone(),
            prompt_diff(diff_content),
        )?;
        println!("{}", diff_explain);

        println!("============================================================================");
        println!("Generating commit message...\n");
        cm_msg = chat(
            G_AI_MODEL.clone(),
            G_AI_API_KEY.clone(),
            prompt_create_commit_msg(diff_explain),
        )?;
        println!("{}", cm_msg);
    }

    git_commit(signoff, directly, cm_msg)?;

    Ok(())
}

fn handle_list(_number: Option<u32>, _explain: bool) -> Result<(), Box<dyn Error>> {
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
            handle_diff(index.unwrap_or("HEAD".to_string()), explain)?;
        }
        Some(Commands::Commit {
            explain,
            signoff,
            direct,
        }) => {
            handle_commit(explain, signoff, direct)?;
        }
        Some(Commands::List { number, explain }) => {
            handle_list(number, explain)?;
        }
        Some(Commands::Show { hash, explain }) => {
            handle_show(hash.unwrap_or("HEAD".to_string()), explain)?;
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
