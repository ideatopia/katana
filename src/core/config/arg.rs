use std::env::args;
use std::path::PathBuf;
use crate::core::utils::logger::LogLevel;
use super::config::Config;

pub fn load_args() -> Config {
    let env_args: Vec<String> = args().collect();
    parse_args(env_args)
}

pub fn parse_args(args: Vec<String>) -> Config {
    let mut host = None;
    let mut port = None;
    let mut root_dir = None;
    let mut worker = None;
    let mut log_level = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--port" => {
                if i + 1 < args.len() {
                    port = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--dir" => {
                if i + 1 < args.len() {
                    root_dir = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--host" => {
                if i + 1 < args.len() {
                    host = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--worker" => {
                if i + 1 < args.len() {
                    worker = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--log-level" => {
                if i + 1 < args.len() {
                    log_level = LogLevel::from_str(&args[i + 1].to_uppercase());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    Config {
        host: host.unwrap_or_default(),
        port: port.unwrap_or_default(),
        root_dir: root_dir.unwrap_or_default(),
        worker: worker.unwrap_or_default(),
        log_level: log_level.unwrap_or(LogLevel::INFO),
    }
}
