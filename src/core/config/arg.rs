use std::env::args;
use std::path::PathBuf;
use crate::core::utils::logger::{Logger, LogLevel};
use super::default::DefaultConfig;
use super::config::Config;

pub fn load_args() -> Config {
    let env_args: Vec<String> = args().collect();
    parse_args(env_args)
}

pub fn parse_args(args: Vec<String>) -> Config {
    let mut host = if cfg!(target_family = "windows") {
        DefaultConfig::HOST_WINDOWS.to_string()
    } else {
        DefaultConfig::HOST_UNIX.to_string()
    };
    let mut port = DefaultConfig::PORT;
    let mut root_dir = PathBuf::from(DefaultConfig::ROOT_DIR);
    let mut worker = DefaultConfig::WORKER;
    let mut log_level = DefaultConfig::LOG_LEVEL;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--port" => {
                if i + 1 < args.len() {
                    port = args[i + 1].parse().unwrap_or(DefaultConfig::PORT);
                    i += 1;
                }
            }
            "--dir" => {
                if i + 1 < args.len() {
                    root_dir = PathBuf::from(&args[i + 1]);
                    i += 1;
                }
            }
            "--host" => {
                if i + 1 < args.len() {
                    host.clone_from(&args[i + 1]);
                    i += 1;
                }
            }
            "--worker" => {
                if i + 1 < args.len() {
                    if let Ok(parsed_worker) = args[i + 1].parse::<i32>() {
                        if parsed_worker > DefaultConfig::MIN_WORKER {
                            worker = parsed_worker;
                        } else {
                            Logger::error("worker cannot be less than 1");
                        }
                    }
                    i += 1;
                }
            }
            "--log-level" => {
                if i + 1 < args.len() {
                    log_level =
                        LogLevel::from_str(&args[i + 1].to_uppercase()).unwrap_or_else(|| {
                            Logger::warn(
                                format!("Invalid log level '{}', using default", args[i + 1])
                                    .as_str(),
                            );
                            DefaultConfig::LOG_LEVEL
                        });
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    Config {
        host,
        port,
        root_dir,
        worker,
        log_level,
    }
}
