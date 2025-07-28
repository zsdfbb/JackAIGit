use std::error::Error;
use clap::Parser;
use git2::{DiffFormat, DiffOptions, Repository};
use log::{debug, info};

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    // subcommand
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Show the diff between the working tree and the index
    Diff {
        /// specify the commit index such as HEAD^
        index: String,
        /// explain the diff between the working tree and the index
        #[arg(short, long)] explain: bool,
    },
    /// Commit the current changes
    Commit {
        /// specify the commit message
        #[arg(short, long)] message: Option<String>,
        /// generate commit message
        #[arg(short, long)] explain: bool,
        /// specify the git commit message template
        #[arg(short, long)] template: Option<String>,
    },
    /// List all commits
    List {
        /// number of commits
        #[arg(short, long)] number: Option<u32>,
        /// explain selected commit
        #[arg(short, long)] explain: bool,
    },
}

fn handle_diff(repo: Repository, _index: String, explain: bool) {
    // 获取 HEAD 引用
    let head = repo.head().expect("Cannot get git HEAD.");
    let head_tree = head.peel_to_tree().expect("Cannot peel to tree.");
    
    // 获取索引（暂存区）
    let mut index = repo.index().expect("Failed to get git repo index");

    let mut diff_options = DiffOptions::new();
    diff_options
        .include_untracked(true)   // 包含未跟踪文件
        .recurse_untracked_dirs(true) // 递归未跟踪目录
        .context_lines(3);          // 上下文行数


    // 获取工作目录与暂存区之间的差异（未暂存的修改）
    let diff = repo.diff_index_to_workdir(
        Some(&mut index),
        Some(&mut diff_options)
    ).expect("Failed to get diff");

    // 打印 diff 结果
    diff.print(DiffFormat::Patch, |delta, _hunk, line| {
        let line_content = std::str::from_utf8(line.content()).unwrap_or("");
        match line.origin() {
            '+' => print!("+{}", line_content),
            '-' => print!("-{}", line_content),
            '>' => print!(">{}", line_content),  // 重命名/复制
            _ => print!(" {}", line_content),    // 其他情况
        }
        true
    }).expect("Failed to iterate over diff");
}

pub fn handle() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let repo = git2::Repository::open(".").expect("Open git repository failed.");

    match cli.command {
        Commands::Diff {index, explain} => {
            handle_diff(repo, index, explain);
        }
        Commands::Commit { message, explain , template} => {
            
        }
        Commands::List { number, explain } => {
           
        }
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
        Commands::Diff { index , explain} => {
            println!("Diff index: {}, explain: {:?}", index, explain);
        }
        Commands::Commit { message, explain } => {
            println!("Commit specify message: {:?}, explain: {:?}", message, explain);
        },
        Commands::List { number, explain } => {
            println!("List num: {:?}, explain: {:?}", number, explain);
        },
    }
}

#[cfg(feature = "test")]
pub fn test() {
    test_options();
    test_parse_subcommand();
}