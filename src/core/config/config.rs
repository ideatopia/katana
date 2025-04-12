use std::path::PathBuf;
use crate::core::utils::logger::LogLevel;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub root_dir: PathBuf,
    pub worker: i32,
    pub log_level: LogLevel,
}

impl Config {
    pub fn load_args() -> Self {
        super::arg::load_args()
    }
    
    pub fn load_env() -> Self {
        super::env::load_env()
    }
}
