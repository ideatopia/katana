use std::fs;
use std::path::PathBuf;
use crate::core::config::config::Config;
use crate::core::config::default::DefaultConfig;
use crate::core::utils::logger::{Logger, LogLevel};
use crate::core::utils::toml::{TomlParser, TomlValue};

pub fn load_file() -> Config {
    // Use the root directory of the project
    let document_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let katana_file_path = document_root.join(".katana");

    let mut parser = TomlParser::new();

    if !katana_file_path.exists() {
        println!("{:?}", katana_file_path);
        Logger::warn("[Config:File] No .katana file found in the root directory");
        return DefaultConfig::as_config();
    }

    let toml_string = fs::read_to_string(&katana_file_path).unwrap_or_else(|_| {
        Logger::error("[Config:File] Failed to read .katana file");
        String::new()
    });

    parser.parse(&toml_string);

    // Extracting configuration values from the parser
    let katana = parser.get_value("katana");

    let host = if let Some(TomlValue::Table(table)) = katana {
        match table.get("host") {
            Some(TomlValue::String(h)) => h.clone(),
            _ => {
                if cfg!(target_family = "windows") {
                    DefaultConfig::HOST_WINDOWS.to_string()
                } else {
                    DefaultConfig::HOST_UNIX.to_string()
                }
            }
        }
    } else {
        if cfg!(target_family = "windows") {
            DefaultConfig::HOST_WINDOWS.to_string()
        } else {
            DefaultConfig::HOST_UNIX.to_string()
        }
    };

    let port = if let Some(TomlValue::Table(table)) = katana {
        match table.get("port") {
            Some(TomlValue::Integer(p)) => *p as u16,
            _ => DefaultConfig::PORT,
        }
    } else {
        DefaultConfig::PORT
    };

    let document_root = if let Some(TomlValue::Table(table)) = katana {
        match table.get("document_root") {
            Some(TomlValue::String(dir)) => PathBuf::from(dir),
            _ => PathBuf::from(DefaultConfig::DOCUMENT_ROOT),
        }
    } else {
        PathBuf::from(DefaultConfig::DOCUMENT_ROOT)
    };

    let worker = if let Some(TomlValue::Table(table)) = katana {
        match table.get("worker") {
            Some(TomlValue::Integer(w)) => *w as i32,
            _ => DefaultConfig::WORKER,
        }
    } else {
        DefaultConfig::WORKER
    };

    let log_level = if let Some(TomlValue::Table(table)) = katana {
        if let Some(TomlValue::String(level)) = table.get("log_level") {
            LogLevel::from_str(&level.to_uppercase()).unwrap_or(DefaultConfig::LOG_LEVEL)
        } else {
            DefaultConfig::LOG_LEVEL
        }
    } else {
        DefaultConfig::LOG_LEVEL
    };

    Config {
        _source: crate::core::config::config::ConfigSource::File,
        host,
        port,
        document_root,
        worker,
        log_level,
    }
}
