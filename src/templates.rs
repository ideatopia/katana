use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
pub enum TemplatesPage {
    BANNER,
    ERROR,
    DIRECTORY,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Templates {
    pub banner: String,
    pub error: String,
    pub directory: String,
}

impl Templates {
    pub fn load() -> Self {
        let templates = Templates {
            banner: String::from(include_str!("../templates/banner.txt")),
            error: String::from(include_str!("../templates/error.html")),
            directory: String::from(include_str!("../templates/directory.html")),
        };

        return templates;
    }

    pub fn from_enum(template_page: TemplatesPage) -> Option<String> {
        let templates = Self::load();

        match template_page {
            TemplatesPage::BANNER => Some(templates.banner),
            TemplatesPage::ERROR => Some(templates.error),
            TemplatesPage::DIRECTORY => Some(templates.directory),
        }
    }

    pub fn render(&self, template: &str, params: HashMap<String, String>) -> String {
        let mut content = match template {
            "banner" => self.banner.clone(),
            "error" => self.error.clone(),
            "directory" => self.directory.clone(),
            _ => String::new(),
        };

        for (key, value) in params {
            let placeholder = "{{".to_string() + &key + "}}";

            if value.is_empty() {
                continue;
            }

            content = content.replace(&placeholder, &value);
        }

        return content;
    }
}
