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
    pub fn load() -> Self {
        // config sources in priority order
        let configs = vec![
            Self::load_file(), // load .katana file, but if file not exist, return default config
            Self::load_env(),
            Self::load_args(),
        ];

        configs.into_iter().fold(Config::default(), |acc, curr| {
            Config {
                host: if curr.host.is_empty() { acc.host } else { curr.host },
                port: if curr.port == 0 { acc.port } else { curr.port },
                root_dir: if curr.root_dir.as_os_str().is_empty() { acc.root_dir } else { curr.root_dir },
                worker: if curr.worker <= 0 { acc.worker } else { curr.worker },
                log_level: curr.log_level,
            }
        })
    }

    fn default() -> Self {
        super::default::DefaultConfig::as_config()
    }

    pub fn load_args() -> Self {
        super::arg::load_args()
    }
    
    pub fn load_env() -> Self {
        super::env::load_env()
    }

    pub fn load_file() -> Self {
        super::file::load_file()
    }
}
