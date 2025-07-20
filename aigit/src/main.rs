use std::env;
use log::{debug, info, error};

mod ollama;

const VERSION: &str = "1.0.0";

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


fn main() {
    logger_init(String::from("debug"));
}