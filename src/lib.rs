use std::collections::HashMap;
use crate::config::Config;
use crate::logger::{Logger, LogLevel};
use crate::server::Server;
use crate::templates::{Templates, TemplatesPage};

pub mod config;
pub mod logger;
pub mod utils;
pub mod templates;
pub mod http;
pub mod server;
pub mod request;
pub mod response;
pub mod filetype;

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
        self.show_banner();
        let server = Server::new(self.config.to_owned(), self.templates.to_owned());
        Logger::log(LogLevel::INFO, format!("Server starting on {}", server.addr_with_protocol()).as_str());
        server.serve();
    }

    fn show_banner(&self) {
        let mut params = HashMap::new();
        params.insert("version".to_string(), format!("{: >1$}", Server::version(), 67));
        println!("{}", self.templates.render(TemplatesPage::BANNER, params));
    }
}
