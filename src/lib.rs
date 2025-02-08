use crate::templates::Templates;

pub mod config;
pub mod logger;
pub mod templates;
pub mod http;

pub struct Katana {
    pub templates: Templates,
}

impl Katana {
    pub fn new() -> Self {
        return Self {
            templates: Templates::load(),
        };
    }

    pub fn start(&self) {
        Self::show_banner(self);
    }

    fn show_banner(&self) {
        println!("{}", self.templates.banner);
    }
}
