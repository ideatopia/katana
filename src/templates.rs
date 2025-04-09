use crate::logger::Logger;
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
        Logger::debug("[Templates] Loading template files");
        let templates = Templates {
            banner: String::from(include_str!("../templates/banner.txt")),
            error: String::from(include_str!("../templates/error.html")),
            directory: String::from(include_str!("../templates/directory.html")),
        };
        Logger::debug("[Templates] Template files loaded successfully");
        templates
    }

    pub fn from_enum(template_page: TemplatesPage) -> Option<String> {
        Logger::debug(
            format!(
                "[Templates] Loading template from enum: {:?}",
                template_page
            )
            .as_str(),
        );
        let templates = Self::load();

        let result = match template_page {
            TemplatesPage::BANNER => Some(templates.banner),
            TemplatesPage::ERROR => Some(templates.error),
            TemplatesPage::DIRECTORY => Some(templates.directory),
        };

        if result.is_none() {
            Logger::warn(
                format!("[Templates] Template not found for {:?}", template_page).as_str(),
            );
        }
        result
    }

    pub fn render(&self, template: TemplatesPage, params: HashMap<String, String>) -> String {
        Logger::debug(
            format!(
                "[Templates] Rendering template {:?} with {} parameters",
                template,
                params.len()
            )
            .as_str(),
        );

        let mut content = match Self::from_enum(template) {
            Some(content) => content,
            None => {
                Logger::error("[Templates] Failed to load template");
                panic!("Cannot load unregistered template");
            }
        };

        for (key, value) in params {
            if value.is_empty() {
                Logger::debug(
                    format!("[Templates] Skipping empty value for key: {}", key).as_str(),
                );
                continue;
            }

            let placeholder = "{{".to_string() + &key + "}}";
            content = content.replace(&placeholder, &value);
        }

        Logger::debug("[Templates] Template rendered successfully");
        content
    }
}
