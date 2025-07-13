use std::path::PathBuf;
use crate::core::utils::logger::{Logger, LogLevel};

#[derive(Clone, Debug)]
pub enum ConfigSource {
    Default,
    File,
    Env,
    Args,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub _display_help: bool,
    pub _source: ConfigSource,
    pub host: String,
    pub port: u16,
    pub document_root: PathBuf,
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

        let config = configs.into_iter().fold(Self::default(), |acc, curr| {
            Config {
                _display_help: acc._display_help || curr._display_help,
                _source: curr._source,
                host: if curr.host.is_empty() { acc.host } else { curr.host },
                port: if curr.port == 0 { acc.port } else { curr.port },
                document_root: if curr.document_root.as_os_str().is_empty() { acc.document_root } else { curr.document_root },
                worker: if curr.worker <= 0 { acc.worker } else { curr.worker },
                log_level: curr.log_level,
            }
        });

        Logger::debug(
            format!(
                "[Config] Configuration from {:?}: host={:?}, port={:?}, root_dir={:?}, worker={:?}, log_level={:?}",
                config._source, config.host, config.port, config.document_root, config.worker, config.log_level
            ).as_str(),
        );

        config
    }

    fn default() -> Self {
        super::default::load_default()
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
