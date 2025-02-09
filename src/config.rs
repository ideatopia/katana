use std::env::args;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub root_dir: PathBuf,
}

impl Config {
    pub fn load_args() -> Self {
        let env_args: Vec<String> = args().collect();
        return Self::parse_args(env_args);
    }

    pub fn parse_args(args: Vec<String>) -> Self {
        let mut host = if cfg!(target_family = "windows") {
            "127.0.0.1".to_string()
        } else {
            "0.0.0.0".to_string()
        };
        let mut port = 8080;
        let mut root_dir = PathBuf::from("public");

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
                _ => {}
            }
            i += 1;
        }

        Config { host, port, root_dir }
    }
}
