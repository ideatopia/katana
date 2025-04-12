use crate::core::utils::logger::LogLevel;

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
}