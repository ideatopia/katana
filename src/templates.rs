use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TemplatesPage {
    BANNER,
    ERROR,
    DIRECTORY,
}

#[derive(Debug, Clone)]
pub struct Templates {
    pub banner: String,
    pub error: String,
    pub directory: String,
}

impl Templates {
    pub fn load() -> Self {
        Templates {
            banner: String::from(include_str!("../templates/banner.txt")),
            error: String::from(include_str!("../templates/error.html")),
            directory: String::from(include_str!("../templates/directory.html")),
        }
    }

    pub fn from_enum(template_page: TemplatesPage) -> Option<String> {
        let templates = Self::load();

        match template_page {
            TemplatesPage::BANNER => Some(templates.banner),
            TemplatesPage::ERROR => Some(templates.error),
            TemplatesPage::DIRECTORY => Some(templates.directory),
        }
    }

    pub fn render(&self, template: TemplatesPage, params: HashMap<String, String>) -> String {
        let mut content = Self::from_enum(template).expect("Cannot load unregistered template");

        for (key, value) in params {
            let placeholder = "{{".to_string() + &key + "}}";

            if value.is_empty() {
                continue;
            }

            content = content.replace(&placeholder, &value);
        }

        content
    }
}
