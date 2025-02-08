use crate::config::Config;
use crate::logger::{Logger, LogLevel};
use crate::templates::Templates;

pub mod config;
pub mod logger;
pub mod templates;
pub mod http;

pub struct Katana {
    pub config: Config,
    pub templates: Templates,
}

impl Katana {
    pub fn new() -> Self {
        return Self {
            config: Config::load_args(),
            templates: Templates::load(),
        };
    }

    pub fn start(&self) {
        Self::show_banner(self);
        Logger::log(LogLevel::INFO, format!("Server running on {}", self.server_address()).as_str());
    }

    fn show_banner(&self) {
        println!("{}", self.templates.banner);
    }

    fn server_address(&self) -> String {
        format!("http://{}:{}", self.config.host, self.config.port)
    }
}
