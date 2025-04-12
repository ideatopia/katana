use std::path::PathBuf;
use crate::core::utils::logger::LogLevel;
use super::config::Config;

pub struct DefaultConfig;

impl DefaultConfig {
    pub const HOST_WINDOWS: &'static str = "127.0.0.1";
    pub const HOST_UNIX: &'static str = "0.0.0.0";
    pub const PORT: u16 = 8080;
    pub const ROOT_DIR: &'static str = "public";
    pub const WORKER: i32 = 4;
    pub const MIN_WORKER: i32 = 1;
    pub const CHUNK_SIZE: usize = 8192;
    pub const LOG_LEVEL: LogLevel = LogLevel::INFO;

    pub fn as_config() -> Config {
        Config {
            host: None::<String>.unwrap_or_else(|| {
                if cfg!(target_family = "windows") {
                    Self::HOST_WINDOWS.to_string()
                } else {
                    Self::HOST_UNIX.to_string()
                }
            }),
            port: None::<u16>.unwrap_or(Self::PORT),
            root_dir: None::<PathBuf>.unwrap_or_else(|| PathBuf::from(Self::ROOT_DIR)),
            worker: None::<i32>.unwrap_or(Self::WORKER),
            log_level: None::<LogLevel>.unwrap_or(Self::LOG_LEVEL),
        }
    }
}