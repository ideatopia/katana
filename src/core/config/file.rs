use std::fs;
use std::path::PathBuf;
use crate::core::config::config::Config;
use crate::core::config::default::load_default;
use crate::core::utils::logger::{Logger, LogLevel};
use crate::core::utils::toml::{TomlParser, TomlValue};

pub fn load_file() -> Config {
    // Use the root directory of the project
    let document_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let katana_file_path = document_root.join(".katana");

    let mut parser = TomlParser::new();

    let default_config = load_default();

    if !katana_file_path.exists() {
        println!("{:?}", katana_file_path);
        Logger::warn("[Config:File] No .katana file found in the root directory");
        return default_config;
    }

    let toml_string = fs::read_to_string(&katana_file_path).unwrap_or_else(|_| {
        Logger::error("[Config:File] Failed to read .katana file");
        String::new()
    });

    parser.parse(&toml_string);

    // Extracting configuration values from the parser
    let katana = match parser.get_value("katana") {
        Some(TomlValue::Table(t)) => t,
        _ => &std::collections::HashMap::new(), // use keyval later
    };

    let host = match katana.get("host") {
        Some(TomlValue::String(h)) => h.clone(),
        _ => default_config.host.clone(),
    };

    let port = match katana.get("port") {
        Some(TomlValue::Integer(p)) => *p as u16,
        _ => default_config.port,
    };

    let document_root = match katana.get("document_root") {
        Some(TomlValue::String(dir)) => PathBuf::from(dir),
        _ => default_config.document_root.clone(),
    };

    let worker = match katana.get("worker") {
        Some(TomlValue::Integer(w)) => *w as i32,
        _ => default_config.worker,
    };

    let log_level = match katana.get("log_level") {
        Some(TomlValue::String(level)) => LogLevel::from_str(&level.to_uppercase()).unwrap_or(default_config.log_level),
        _ => default_config.log_level,
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
