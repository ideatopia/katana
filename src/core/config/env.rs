use std::env;
use std::path::PathBuf;
use crate::core::utils::logger::LogLevel;
use super::config::Config;

pub fn load_env() -> Config {
    let host = env::var("KATANA_HOST")
        .map(|h| h.to_string())
        .ok();

    let port = env::var("KATANA_PORT")
        .ok()
        .and_then(|p| p.parse().ok());

    let document_root = env::var("KATANA_DOCUMENT_ROOT")
        .map(PathBuf::from)
        .ok();

    let worker = env::var("KATANA_WORKER")
        .ok()
        .and_then(|w| w.parse::<i32>().ok());

    let log_level = env::var("KATANA_LOG_LEVEL")
        .ok()
        .and_then(|l| LogLevel::from_str(&l.to_uppercase()));

    Config {
        host: host.unwrap_or_default(),
        port: port.unwrap_or_default(),
        document_root: document_root.unwrap_or_default(),
        worker: worker.unwrap_or_default(),
        log_level: log_level.unwrap_or(LogLevel::DEBUG),
    }
}
