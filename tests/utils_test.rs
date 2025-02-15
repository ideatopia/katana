use katana::utils::Utils;
use std::env;
use std::fs::{self, File};
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a temporary directory and ensure it is cleaned up after test
    fn create_temp_dir() -> PathBuf {
        let temp_dir = env::temp_dir().join("utils_test_temp_dir");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();
        temp_dir
    }

    /// Test `walk_dir` with non-existent path
    #[test]
    fn test_walk_dir_non_existent_path() {
        let path = PathBuf::from("/non/existent/path");
        let result = Utils::walk_dir(&path);
        assert!(
            result.is_empty(),
            "Expected no results for non-existent path"
        );
    }

    /// Test `walk_dir` with hidden files
    #[test]
    fn test_walk_dir_with_hidden_files() {
        let temp_dir = create_temp_dir();
        let hidden_file = temp_dir.join(".hidden_file.txt");
        let normal_file = temp_dir.join("normal_file.txt");
        File::create(normal_file).unwrap();
        File::create(hidden_file).unwrap();

        let result = Utils::walk_dir(&temp_dir);

        assert_eq!(
            result.len(),
            1,
            "Expected only the normal file to be included"
        );
        assert_eq!(
            result[0].1, "normal_file.txt",
            "Expected normal_file.txt to be listed"
        );
    }

    /// Test `walk_dir` with symbolic link (only on Unix-like systems)
    #[cfg(unix)]
    #[test]
    fn test_walk_dir_with_symlink() {
        let temp_dir = create_temp_dir();
        let target_file = temp_dir.join("target_file.txt");
        let symlink = temp_dir.join("symlink_file.txt");

        // Create target file and symlink
        File::create(&target_file).unwrap();
        std::os::unix::fs::symlink(&target_file, &symlink).unwrap();

        let result = Utils::walk_dir(&temp_dir);

        assert_eq!(
            result.len(),
            2,
            "Expected both the target file and the symlink to be listed"
        );
        assert!(result.iter().any(|entry| entry.1 == "target_file.txt"));
        assert!(result.iter().any(|entry| entry.1 == "symlink_file.txt"));
    }

    /// Test `normalize_path` with complex paths
    #[test]
    fn test_normalize_path_with_complex_path() {
        let path = PathBuf::from("folder/.././folder2/./file.txt");
        let normalized_path = Utils::normalize_path(path);
        assert_eq!(
            normalized_path,
            PathBuf::from("folder2/file.txt"),
            "Path normalization failed"
        );
    }

    /// Test `normalize_path` with multiple separators
    #[test]
    fn test_normalize_path_with_multiple_separators() {
        let path = PathBuf::from("folder//subfolder//file.txt");
        let normalized_path = Utils::normalize_path(path);
        assert_eq!(
            normalized_path,
            PathBuf::from("folder/subfolder/file.txt"),
            "Multiple separators should be normalized"
        );
    }

    /// Test `normalize_path` with `..` and `.` in the path
    #[test]
    fn test_normalize_path_with_parent_current_dirs() {
        let path = PathBuf::from("folder/./subfolder/../file.txt");
        let normalized_path = Utils::normalize_path(path);
        assert_eq!(
            normalized_path,
            PathBuf::from("folder/file.txt"),
            "Path normalization with `..` and `.` failed"
        );
    }

    /// Test `collect_entries` with empty directory
    #[test]
    fn test_collect_entries_with_empty_directory() {
        let temp_dir = create_temp_dir();
        let entries = fs::read_dir(temp_dir).unwrap();
        let result = Utils::collect_entries(entries);
        assert!(
            result.is_empty(),
            "Expected empty result for empty directory"
        );
    }

    /// Test `is_valid_entry` for hidden files
    #[test]
    fn test_is_valid_entry_with_hidden_files() {
        assert!(
            !Utils::is_valid_entry(".hidden_file"),
            "Hidden file should not be valid"
        );
        assert!(
            Utils::is_valid_entry("visible_file"),
            "Visible file should be valid"
        );
    }

    /// Clean up created temporary directory after tests
    fn cleanup_temp_dir() {
        let temp_dir = env::temp_dir().join("utils_test_temp_dir");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
    }

    /// Run the cleanup function after each test
    #[test]
    fn run_cleanup_after_tests() {
        cleanup_temp_dir();
    }
}
