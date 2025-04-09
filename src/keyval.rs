use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct KeyVal {
    map: HashMap<String, String>,
}

impl Default for KeyVal {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyVal {
    pub fn new() -> Self {
        KeyVal {
            map: HashMap::new(),
        }
    }

    pub fn map(&self) -> &HashMap<String, String> {
        &self.map
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

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, String> {
        self.map.iter_mut()
    }
}
