use std::fs::{self, ReadDir};
use std::path::{Component, PathBuf};

#[derive(Debug)]
pub struct Utils;

impl Utils {
    pub fn walk_dir(path: &PathBuf) -> Vec<(String, String, String)> {
        let mut results = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in Self::collect_entries(entries) {
                if let Ok(metadata) = entry.metadata() {
                    if let Some(name) = entry.file_name().to_str() {
                        if Self::is_valid_entry(name) {
                            let mut entry_path = entry.path().to_string_lossy().replace('\\', "/");
                            let entry_type = if metadata.is_dir() { "directory" } else { "file" };
                            if metadata.is_dir() && !entry_path.ends_with("/") {
                                entry_path.insert_str(entry_path.len(), "/");
                            }
                            results.push((entry_type.to_string(), name.to_string(), entry_path));
                        }
                    }
                }
            }
        }
        results
    }

    pub fn collect_entries(entries: ReadDir) -> Vec<fs::DirEntry> {
        entries.filter_map(|entry| entry.ok()).collect()
    }

    pub fn is_valid_entry(name: &str) -> bool {
        !name.starts_with('.')
    }

    pub fn normalize_path(path: PathBuf) -> PathBuf {
        let mut normalized = PathBuf::new();
        for component in path.components() {
            match component {
                Component::ParentDir => { normalized.pop(); },
                Component::CurDir => {},
                _ => normalized.push(component.as_os_str()),
            }
        }
        PathBuf::from(normalized.to_string_lossy().replace('\\', "/"))
    }
}
