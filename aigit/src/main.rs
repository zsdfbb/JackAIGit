use std::process::exit;
use std::error::Error;

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
fn open_git_repo() -> Result<git2::Repository, git2::Error> {
    let repo = git2::Repository::open(".")?;
    Ok(repo)
}

fn main() -> Result<(), Box<dyn Error>> {
    logger_init(String::from("debug"));

    // 是否定义了 test #![feature()] 参数
    if cfg!(feature = "test") {
        info!("test mode enabled!");

        debug!("test command parse...");
        command::test();

        /*
        debug!("test ollama api...");
        let _test = ollama::test();
        */
        exit(0);
    }

    open_git_repo()?;

    let ret = command::handle();
    if ret.is_err() {
        error!("{:?}", ret);
        exit(1);
    }
    exit(0);
}