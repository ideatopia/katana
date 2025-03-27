use std::io::Write;
use crate::utils::Utils;

#[derive(Debug)]
#[allow(dead_code)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
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
}

pub struct Logger;

impl Logger {
    pub fn debug(message: &str) {
        let log_message = Self::build_log_message(LogLevel::DEBUG, message);
        println!("{}", log_message);
    }

    pub fn info(message: &str) {
        let log_message = Self::build_log_message(LogLevel::INFO, message);
        println!("{}", log_message);
    }

    pub fn warn(message: &str) {
        let log_message = Self::build_log_message(LogLevel::WARN, message);
        println!("{}", log_message);
    }

    pub fn error(message: &str) {
        let log_message = Self::build_log_message(LogLevel::ERROR, message);
        println!("{}", log_message);
    }

    pub fn log(level: LogLevel, message: &str) {
        let log_message = Self::build_log_message(level, message);
        println!("{}", log_message);
    }

    pub fn writer<W: Write>(level: LogLevel, message: &str, writer: &mut W) {
        let log_message = Self::build_log_message(level, message);
        let _ = writer.write_all(log_message.as_bytes()); // ignoring errors for simplicity
    }

    fn build_log_message(level: LogLevel, message: &str) -> String {
        let at = Utils::log_datetime();
        let level_str = level.as_str();
        let log_message = format!("[{}] [{}] {}", at, level_str, message);
        log_message
    }
}
