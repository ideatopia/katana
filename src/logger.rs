use crate::colorful::Colored;
use crate::config::Config;
use crate::utils::Utils;
use std::io::Write;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[allow(dead_code)]
pub enum LogLevel {
    DEBUG = 0,
    INFO = 1,
    WARN = 2,
    ERROR = 3,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::DEBUG => "DEBUG",
            LogLevel::INFO => "INFO",
            LogLevel::WARN => "WARN",
            LogLevel::ERROR => "ERROR",
        }
    }

    pub fn should_log(&self, min_level: &LogLevel) -> bool {
        self >= min_level
    }

    pub fn from_str(level: &str) -> Option<Self> {
        match level {
            "DEBUG" => Some(LogLevel::DEBUG),
            "INFO" => Some(LogLevel::INFO),
            "WARN" => Some(LogLevel::WARN),
            "ERROR" => Some(LogLevel::ERROR),
            _ => None,
        }
    }
}

pub struct Logger;

impl Logger {
    pub fn get_min_log_level() -> LogLevel {
        let config = Config::load_args();
        config.log_level
    }

    pub fn debug(message: &str) {
        let level = LogLevel::DEBUG;
        if !level.should_log(&Self::get_min_log_level()) {
            return;
        }
        let log_message = Self::build_log_message(level, message);
        println!("{}", log_message);
    }

    pub fn info(message: &str) {
        let level = LogLevel::INFO;
        if !level.should_log(&Self::get_min_log_level()) {
            return;
        }
        let log_message = Self::build_log_message(level, message);
        println!("{}", log_message);
    }

    pub fn warn(message: &str) {
        let level = LogLevel::WARN;
        if !level.should_log(&Self::get_min_log_level()) {
            return;
        }
        let log_message = Self::build_log_message(level, message);
        println!("{}", log_message);
    }

    pub fn error(message: &str) {
        let level = LogLevel::ERROR;
        if !level.should_log(&Self::get_min_log_level()) {
            return;
        }
        let log_message = Self::build_log_message(level, message);
        println!("{}", log_message);
    }

    pub fn log(level: LogLevel, message: &str) {
        if !level.should_log(&Self::get_min_log_level()) {
            return;
        }
        let log_message = Self::build_log_message(level, message);
        println!("{}", log_message);
    }

    pub fn writer<W: Write>(level: LogLevel, message: &str, writer: &mut W) {
        if !level.should_log(&Self::get_min_log_level()) {
            return;
        }
        let log_message = Self::build_log_message(level, message);
        let _ = writer.write_all(log_message.as_bytes());
    }

    fn colorful_log_level(level: LogLevel) -> String {
        let colored = match level {
            LogLevel::DEBUG => level.as_str().cyan().dim(),
            LogLevel::INFO => level.as_str().green(),
            LogLevel::WARN => level.as_str().yellow().bold(),
            LogLevel::ERROR => level.as_str().red().bold(),
        };

        colored.to_string()
    }

    fn build_log_message(level: LogLevel, message: &str) -> String {
        let at = Utils::log_datetime();
        let level_binding = Self::colorful_log_level(level);
        let level_str = level_binding.as_str();
        let log_message = format!("[{}] [{}] {}", at, level_str, message);
        log_message
    }
}
