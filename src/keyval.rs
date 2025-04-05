use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct KeyVal {
    map: HashMap<String, String>,
}

impl KeyVal {
    pub fn new() -> Self {
        KeyVal {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    pub fn del(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }

    pub fn exists(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut String> {
        self.map.get_mut(key)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}
