use crate::config::Config;
use crate::logger::{Logger, LogLevel};
use crate::server::Server;
use crate::templates::Templates;

pub mod config;
pub mod logger;
pub mod templates;
pub mod http;
pub mod server;
pub mod request;
pub mod response;

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

        let server = Server::new(
            self.config.host.to_owned(),
            self.config.port.to_owned(),
            self.config.root_dir.to_owned()
        );

        Logger::log(LogLevel::INFO, format!("Server starting on {}", server.addr_with_protocol()).as_str());

        server.serve();
    }

    fn show_banner(&self) {
        println!("{}", self.templates.banner);
    }
}
