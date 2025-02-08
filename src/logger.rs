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
            LogLevel::ERROR => "ERROR"
        }
    }
}

pub struct Logger;

impl Logger {
    pub fn log(level: LogLevel, message: &str) {
        let level_str = LogLevel::from(level).as_str();
        println!("[{}] {}", level_str, message);
    }
}
