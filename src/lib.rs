use crate::config::Config;
use crate::logger::Logger;
use crate::server::Server;
use crate::templates::{Templates, TemplatesPage};
use std::collections::HashMap;

pub mod config;
pub mod filetype;
pub mod http;
pub mod logger;
pub mod request;
pub mod response;
pub mod server;
pub mod templates;
pub mod utils;

pub struct Katana {
    pub config: Config,
    pub templates: Templates,
}

impl Default for Katana {
    fn default() -> Self {
        Self::new()
    }
}

impl Katana {
    pub fn new() -> Self {
        Self {
            config: Config::load_args(),
            templates: Templates::load(),
        }
    }

    pub fn start(&self) {
        self.show_banner();
        let server = Server::new(self.config.to_owned(), self.templates.to_owned());
        Logger::info(
            format!("Server starting on {}", server.addr_with_protocol()).as_str(),
        );
        server.serve();
    }

    fn show_banner(&self) {
        let mut params = HashMap::new();
        params.insert(
            "version".to_string(),
            format!("{: >1$}", Server::version(), 67),
        );
        println!("{}", self.templates.render(TemplatesPage::BANNER, params));
    }
}
