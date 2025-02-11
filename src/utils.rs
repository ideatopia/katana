use std::env;
use std::fs::{self, ReadDir};
use std::path::{Component, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

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

    pub fn timezone_from_env() -> String {
        env::var("TZ").unwrap_or("00:00".to_string())
    }

    pub fn unix_timestamp() -> String {
        if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
            return duration.as_secs().to_string();
        } else {
            return String::new();
        }
    }

    pub fn datetime_rfc_8601() -> String {
        let now = SystemTime::now();
        if let Ok(duration) = now.duration_since(UNIX_EPOCH) {
            let secs = duration.as_secs();

            // Convert seconds to date components using basic arithmetic
            let days_since_epoch = secs / 86400;
            let years_since_epoch = 1970 + (days_since_epoch / 365);
            let mut remaining_days = (days_since_epoch % 365) as i32;

            // Simple month calculation (approximate)
            let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            let mut month = 1;
            for days in month_days.iter() {
                if remaining_days - days <= 0 {
                    break;
                }
                remaining_days -= days;
                month += 1;
            }

            let day = remaining_days + 1;

            // Time components
            let secs_of_day = secs % 86400;
            let hours = (secs_of_day / 3600) % 24;
            let minutes = (secs_of_day % 3600) / 60;
            let seconds = secs_of_day % 60;

            return format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z+{}",
                    years_since_epoch, month, day,
                    hours, minutes, seconds, Self::timezone_from_env());
        } else {
            return String::new();
        }
    }

    pub fn datetime_rfc_1123() -> String {
        let now = SystemTime::now();
        if let Ok(duration) = now.duration_since(UNIX_EPOCH) {
            let secs = duration.as_secs();

            // Convert seconds to date components using basic arithmetic
            let days_since_epoch = secs / 86400;
            let years_since_epoch = 1970 + (days_since_epoch / 365);
            let mut remaining_days = (days_since_epoch % 365) as i32;

            // Simple month calculation (approximate)
            let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            let mut month = 1;
            for days in month_days.iter() {
                if remaining_days - days <= 0 {
                    break;
                }
                remaining_days -= days;
                month += 1;
            }

            let day = remaining_days + 1;

            // Time components
            let secs_of_day = secs % 86400;
            let hours = (secs_of_day / 3600) % 24;
            let minutes = (secs_of_day % 3600) / 60;
            let seconds = secs_of_day % 60;

            // Day of the week and month names
            let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
            let months = [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ];

            // Calculate day of the week (simple Zeller's congruence)
            let mut year = years_since_epoch;
            let mut month = month - 1; // Zero-indexed for Zeller's formula
            if month < 2 {
                month += 12;
                year -= 1;
            }
            let k = (year % 100) as i32; // Cast k to i32
            let j = (year / 100) as i32; // Cast j to i32
            let h = ((day as i32) + ((13 * (month + 1)) / 5) + k + (k / 4) + (j / 4) - (2 * j)) % 7;
            let h = if h < 0 { h + 7 } else { h }; // Ensure h is within 0..=6
            let weekday = weekdays[h as usize % 7]; // Ensure h is within bounds

            let month = month % 12; // Ensure month is within 0..=11
            let monthday = months[month as usize];

            return format!(
                "{}, {:02} {} {:04} {:02}:{:02}:{:02} GMT",
                weekday, day, monthday, years_since_epoch,
                hours, minutes, seconds
            );
        } else {
            String::new() // Return empty string if there's an error
        }
    }
}
