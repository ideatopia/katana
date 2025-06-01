use std::path::PathBuf;
use crate::core::utils::logger::LogLevel;
use super::config::Config;

pub struct DefaultConfig;

impl DefaultConfig {
    pub const HOST_WINDOWS: &'static str = "127.0.0.1";
    pub const HOST_UNIX: &'static str = "0.0.0.0";
    pub const PORT: u16 = 8080;
    pub const DOCUMENT_ROOT: &'static str = "public";
    pub const WORKER: i32 = 4;
    pub const LOG_LEVEL: LogLevel = LogLevel::INFO;

    pub fn as_config() -> Config {
        Config {
            _source: crate::core::config::config::ConfigSource::Default,
            host: None::<String>.unwrap_or_else(|| {
                if cfg!(target_family = "windows") {
                    Self::HOST_WINDOWS.to_string()
                } else {
                    Self::HOST_UNIX.to_string()
                }
            }),
            port: None::<u16>.unwrap_or(Self::PORT),
            document_root: None::<PathBuf>.unwrap_or_else(|| PathBuf::from(Self::DOCUMENT_ROOT)),
            worker: None::<i32>.unwrap_or(Self::WORKER),
            log_level: None::<LogLevel>.unwrap_or(Self::LOG_LEVEL),
        }
    }
}