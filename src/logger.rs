use std::time::{SystemTime, UNIX_EPOCH};

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
        let at = Logger::current_datetime();
        let level_str = LogLevel::from(level).as_str();
        println!("[{}] [{}] {}", at, level_str, message);
    }

    fn current_timestamp() -> u64 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH);
        let timestamp = now.unwrap().as_secs();
        return timestamp;
    }

    fn current_datetime() -> String {
        let seconds = Self::current_timestamp();

        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;

        let datetime = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", 1970 + days / 365, (days % 365) / 30 + 1, days % 30 + 1, hours, minutes, seconds);

        return datetime;
    }
}
