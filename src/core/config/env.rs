use std::env;
use std::path::PathBuf;
use crate::core::utils::logger::{Logger, LogLevel};
use super::config::Config;
use super::default::DefaultConfig;

pub fn load_env() -> Config {
    let host = env::var("KATANA_HOST").unwrap_or_else(|_| {
        if cfg!(target_family = "windows") {
            DefaultConfig::HOST_WINDOWS.to_string()
        } else {
            DefaultConfig::HOST_UNIX.to_string()
        }
    });

    let port = env::var("KATANA_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DefaultConfig::PORT);

    let root_dir = env::var("KATANA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DefaultConfig::ROOT_DIR));

    let worker = env::var("KATANA_WORKER")
        .ok()
        .and_then(|w| w.parse().ok())
        .map(|w: i32| {
            if w > DefaultConfig::MIN_WORKER {
                w
            } else {
                Logger::error("worker cannot be less than 1");
                DefaultConfig::WORKER
            }
        })
        .unwrap_or(DefaultConfig::WORKER);

    let log_level = env::var("KATANA_LOG_LEVEL")
        .map(|l| LogLevel::from_str(&l.to_uppercase()))
        .unwrap_or(Some(DefaultConfig::LOG_LEVEL))
        .unwrap_or_else(|| {
            Logger::warn("Invalid log level in environment variable, using default");
            DefaultConfig::LOG_LEVEL
        });

    Config {
        host,
        port,
        root_dir,
        worker,
        log_level,
    }
}
