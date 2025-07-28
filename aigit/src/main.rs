use std::path::{PathBuf};
use std::process::exit;
use std::error::Error;

use git2::TreeBuilder;
use log::{debug, info, error};

mod command;
mod ollama;

fn logger_init(log_level: String) {
    let mut elog_builder = env_logger::Builder::new();

    if log_level == "debug" {
        elog_builder.filter(None, log::LevelFilter::Debug);
        debug!("Debug mode enabled.")
    } else {
        elog_builder.filter(None, log::LevelFilter::Info);
    }

    elog_builder.init();
}

// open git repo by current directory and git2
fn cur_is_git_repo() -> bool {
    let mut dir: PathBuf = std::env::current_dir().expect("Cannot get current directory.");

    loop {
        // 检查当前层级是否存在 .git
        let git_path: PathBuf = dir.join(".git");
        if git_path.exists() {
            return true;
        }

        // 向上移动到父目录
        if !dir.pop() {
            break; // 到达根目录
        }
    }

    // 返回错误
    false
}


fn main() -> Result<(), Box<dyn Error>> {
    logger_init(String::from("debug"));

    // 是否定义了 test #![feature()] 参数
    if cfg!(feature = "test") {
        info!("test mode enabled!");

        debug!("test command parse...");
        #[cfg(feature = "test")]
        command::test();

        debug!("test ollama api...");
        #[cfg(feature = "test")]
        let _test = ollama::test();

        exit(0);
    }

    if ! cur_is_git_repo() {
        error!("current directory is not a git repository.");
        exit(1);
    } else {
        debug!("current directory is a git repository.")
    }

    let ret = command::handle();
    if ret.is_err() {
        error!("{:?}", ret);
        exit(1);
    }
    exit(0);
}