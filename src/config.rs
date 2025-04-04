use crate::logger::Logger;
use std::env::args;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub root_dir: PathBuf,
    pub worker: i32,
}

impl Config {
    pub const MIN_WORKER: i32 = 1;
    pub const CHUNK_SIZE: usize = 8192;

    pub fn load_args() -> Self {
        let env_args: Vec<String> = args().collect();
        Self::parse_args(env_args)
    }

    pub fn parse_args(args: Vec<String>) -> Self {
        let mut host = if cfg!(target_family = "windows") {
            "127.0.0.1".to_string()
        } else {
            "0.0.0.0".to_string()
        };
        let mut port = 8080;
        let mut root_dir = PathBuf::from("public");
        let mut worker = 4;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--port" => {
                    if i + 1 < args.len() {
                        port = args[i + 1].parse().unwrap_or(8080);
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
                        host = args[i + 1].clone();
                        i += 1;
                    }
                }
                "--worker" => {
                    if i + 1 < args.len() {
                        if let Ok(parsed_worker) = args[i + 1].parse::<i32>() {
                            if parsed_worker > Self::MIN_WORKER {
                                worker = parsed_worker;
                            } else {
                                Logger::error("worker cannot be less than 1");
                            }
                        }
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
        }
    }
}
