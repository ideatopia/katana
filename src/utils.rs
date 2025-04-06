use std::env;
use std::fs::{self, Metadata, ReadDir};
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
                            let entry_type = if metadata.is_dir() {
                                "directory"
                            } else {
                                "file"
                            };
                            if metadata.is_dir() && !entry_path.ends_with('/') {
                                entry_path.insert(entry_path.len(), '/');
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
                Component::ParentDir => {
                    normalized.pop();
                }
                Component::CurDir => {}
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
            duration.as_secs().to_string()
        } else {
            String::new()
        }
    }

    pub fn datetime_rfc_8601() -> String {
        let now = SystemTime::now();
        if let Ok(duration) = now.duration_since(UNIX_EPOCH) {
            let secs = duration.as_secs();
            let millis = duration.subsec_millis(); // Extract milliseconds

            let days_since_epoch = secs / 86400;
            let mut year = 1970;
            let mut days = days_since_epoch as i32;

            // Function to check leap year
            fn is_leap_year(year: i32) -> bool {
                (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
            }

            // Adjust for years
            while days >= (if is_leap_year(year) { 366 } else { 365 }) {
                days -= if is_leap_year(year) { 366 } else { 365 };
                year += 1;
            }

            // Days in each month (adjust for leap year)
            let month_days = [
                31,
                if is_leap_year(year) { 29 } else { 28 },
                31,
                30,
                31,
                30,
                31,
                31,
                30,
                31,
                30,
                31,
            ];

            let mut month = 0;
            while days >= month_days[month] {
                days -= month_days[month];
                month += 1;
            }
            let day = days + 1;

            // Time components
            let secs_of_day = secs % 86400;
            let hours = (secs_of_day / 3600) % 24;
            let minutes = (secs_of_day % 3600) / 60;
            let seconds = secs_of_day % 60;

            format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
                year,
                month + 1, // Convert to 1-based month
                day,
                hours,
                minutes,
                seconds,
                millis // Include milliseconds
            )
        } else {
            String::new()
        }
    }

    pub fn datetime_rfc_1123() -> String {
        let now = SystemTime::now();
        if let Ok(duration) = now.duration_since(UNIX_EPOCH) {
            let secs = duration.as_secs();

            // Convert seconds to date components
            let days_since_epoch = secs / 86400;
            let mut year = 1970;
            let mut days = days_since_epoch as i32;

            // Function to check leap year
            fn is_leap_year(year: i32) -> bool {
                (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
            }

            // Adjust for years
            while days >= (if is_leap_year(year) { 366 } else { 365 }) {
                days -= if is_leap_year(year) { 366 } else { 365 };
                year += 1;
            }

            // Days in each month (adjust for leap year)
            let month_days = [
                31,
                if is_leap_year(year) { 29 } else { 28 },
                31,
                30,
                31,
                30,
                31,
                31,
                30,
                31,
                30,
                31,
            ];
            let month_names = [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ];

            // Find current month and day
            let mut month = 0;
            while days >= month_days[month] {
                days -= month_days[month];
                month += 1;
            }
            let day = days + 1;

            // Compute the day of the week using Zeller’s Congruence
            let mut m: i32 = month as i32 + 1; // Adjusting month to 1-based index
            let mut y: i32 = year;

            if m <= 2 {
                m += 12; // Jan, Feb become 13, 14
                y -= 1; // Adjust year
            }

            let k: i32 = y % 100;
            let j: i32 = y / 100;

            // Apply Zeller’s Congruence formula
            let h: i32 = (day + ((13 * (m + 1)) / 5) + k + (k / 4) + (j / 4) + (5 * j)) % 7;

            // Ensure h is positive
            let h = (h + 7) % 7;

            // Correct mapping (0 = Saturday, 1 = Sunday, ..., 6 = Friday)
            let weekday_names = ["Sat", "Sun", "Mon", "Tue", "Wed", "Thu", "Fri"];
            let weekday = weekday_names[h as usize];

            // Time components
            let secs_of_day = secs % 86400;
            let hours = (secs_of_day / 3600) % 24;
            let minutes = (secs_of_day % 3600) / 60;
            let seconds = secs_of_day % 60;

            // Format as RFC 1123
            format!(
                "{}, {:02} {} {:04} {:02}:{:02}:{:02} GMT",
                weekday, day, month_names[month], year, hours, minutes, seconds
            )
        } else {
            String::new() // Return empty string if there's an error
        }
    }

    pub fn log_datetime() -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH);
        let seconds = now.unwrap().as_secs();

        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;

        let datetime = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            1970 + days / 365,
            (days % 365) / 30 + 1,
            days % 30 + 1,
            hours,
            minutes,
            seconds
        );

        datetime
    }

    pub fn is_readable(path: PathBuf) -> bool {
        let metadata = fs::metadata(path).expect("Unable to read metadata");
        Self::is_readable_from_metadata(metadata)
    }

    pub fn is_readable_from_metadata(metadata: Metadata) -> bool {
        #[cfg(unix)]
        let is_readable = {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            (mode & 0o444) != 0 // check if file has read permission
        };

        #[cfg(windows)]
        let is_readable = {
            !metadata.permissions().readonly() // on Windows, check if file is not readonly
        };

        is_readable
    }

    pub fn path_prettifier(path: PathBuf) -> String {
        // use correct path separator based on system type for display purpose
        #[cfg(target_os = "windows")]
            let prettified = path.to_str().unwrap().replace('/', "\\");

        #[cfg(not(target_os = "windows"))]
            let prettified = path.to_str().unwrap().replace('\\', "/");

        prettified
    }
}
