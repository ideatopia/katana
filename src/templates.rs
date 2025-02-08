use std::collections::HashMap;

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

    pub fn render(&self, template: &str, params: HashMap<String, String>) -> String {
        let mut content = match template {
            "banner" => self.banner.clone(),
            "error" => self.error.clone(),
            "directory" => self.directory.clone(),
            _ => String::new(),
        };

        for (key, value) in params {
            let placeholder = "{{".to_string() + &key + "}}";
            content = content.replace(&placeholder, &value);
        }

        return content;
    }
}
