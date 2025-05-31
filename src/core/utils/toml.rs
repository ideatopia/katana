use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
pub enum TomlValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<TomlValue>),
    Table(HashMap<String, TomlValue>),
}

#[derive(Clone, Debug)]
pub struct TomlParser {
    pub data: HashMap<String, TomlValue>,
}

impl TomlParser {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn parse(&mut self, input: &str) {
        let mut current_section = String::new();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                current_section = line.trim_matches(&['[', ']'][..]).to_string();
                self.data.insert(current_section.clone(), TomlValue::Table(HashMap::new()));
            } else if let Some((key, value)) = line.split_once('=') {
                let mut key = key.trim().to_string();
                if key.starts_with("\"") && key.ends_with("\"") {
                    key = key.trim_matches('"').to_string();
                }
                let value = Self::parse_value(value.trim());
                if let Some(TomlValue::Table(section)) = self.data.get_mut(&current_section) {
                    section.insert(key, value);
                } else {
                    self.data.insert(key, value);
                }
            }
        }
    }

    pub fn parse_value(value: &str) -> TomlValue {
        if value.starts_with('"') && value.ends_with('"') {
            TomlValue::String(value.trim_matches('"').to_string())
        } else if let Ok(int) = value.parse::<i64>() {
            TomlValue::Integer(int)
        } else if let Ok(float) = value.parse::<f64>() {
            TomlValue::Float(float)
        } else if let Ok(boolean) = value.parse::<bool>() {
            TomlValue::Boolean(boolean)
        } else if value.starts_with('[') && value.ends_with(']') {
            let elements = value.trim_matches(&['[', ']'][..])
                .split(',')
                .map(|v| Self::parse_value(v.trim()))
                .collect();
            TomlValue::Array(elements)
        } else {
            TomlValue::String(value.to_string())
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&TomlValue> {
        self.data.get(key)
    }

    pub fn get_nested_value(&self, section: &str, key: &str) -> Option<&TomlValue> {
        if let Some(TomlValue::Table(table)) = self.data.get(section) {
            return table.get(key);
        }
        None
    }
}
