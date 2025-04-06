use katana::config::Config;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn get_host() -> String {
        

        if cfg!(target_family = "windows") {
            "127.0.0.1".to_string()
        } else {
            "0.0.0.0".to_string()
        }
    }

    /// Test case for when no arguments are passed.
    #[test]
    fn test_no_arguments() {
        let args = vec![
            "".to_string(),
            "--port".to_string(),
            "invalid_port".to_string(),
        ];
        let config = Config::parse_args(args);

        assert_eq!(config.host, get_host());
        assert_eq!(config.port, 8080);
        assert_eq!(config.root_dir, PathBuf::from("public"));
        assert_eq!(config.worker, 4);
    }

    /// Test case for passing an invalid port value.
    #[test]
    fn test_invalid_port() {
        let args = vec![
            "".to_string(),
            "--port".to_string(),
            "invalid_port".to_string(),
        ];
        let config = Config::parse_args(args);

        assert_eq!(config.host, get_host());
        assert_eq!(config.port, 8080); // Default port
        assert_eq!(config.root_dir, PathBuf::from("public"));
        assert_eq!(config.worker, 4);
    }

    /// Test case for passing a very large port number.
    #[test]
    fn test_large_port() {
        let args = vec!["".to_string(), "--port".to_string(), "65536".to_string()];
        let config = Config::parse_args(args);

        assert_eq!(config.host, get_host());
        assert_eq!(config.port, 8080); // Default port
        assert_eq!(config.root_dir, PathBuf::from("public"));
        assert_eq!(config.worker, 4);
    }

    /// Test case for passing a very long path string.
    #[test]
    fn test_long_path() {
        let long_path = "a".repeat(300); // generate a long string of 300 characters
        let args = vec!["".to_string(), "--dir".to_string(), long_path];
        let config = Config::parse_args(args);

        assert_eq!(config.host, get_host());
        assert_eq!(config.port, 8080);
        assert_eq!(config.root_dir, PathBuf::from("a".repeat(300)));
        assert_eq!(config.worker, 4);
    }

    /// Test case for passing an invalid host value.
    #[test]
    fn test_invalid_host() {
        let args = vec![
            "".to_string(),
            "--host".to_string(),
            "256.256.256.256".to_string(),
        ];
        let config = Config::parse_args(args);

        assert_eq!(config.host, "256.256.256.256"); // Host accepts any string, no validation
        assert_eq!(config.port, 8080);
        assert_eq!(config.root_dir, PathBuf::from("public"));
        assert_eq!(config.worker, 4);
    }

    /// Test case for passing unexpected arguments.
    #[test]
    fn test_unexpected_arguments() {
        let args = vec![
            "".to_string(),
            "--unknown".to_string(),
            "some_value".to_string(),
        ];
        let config = Config::parse_args(args);

        assert_eq!(config.host, get_host()); // Default should still be used
        assert_eq!(config.port, 8080); // Default should still be used
        assert_eq!(config.root_dir, PathBuf::from("public")); // Default should still be used
        assert_eq!(config.worker, 4);
    }

    /// Test case for passing the port argument multiple times.
    #[test]
    fn test_multiple_port_specifications() {
        let args = vec![
            "".to_string(),
            "--port".to_string(),
            "9090".to_string(),
            "--port".to_string(),
            "5000".to_string(),
        ];
        let config = Config::parse_args(args);

        assert_eq!(config.host, get_host());
        assert_eq!(config.port, 5000); // Last port specified should be used
        assert_eq!(config.root_dir, PathBuf::from("public"));
        assert_eq!(config.worker, 4);
    }

    /// Test case for passing an invalid worker value.
    #[test]
    fn test_invalid_worker() {
        let args = vec!["".to_string(), "--worker".to_string(), "-1".to_string()];
        let config = Config::parse_args(args);

        assert_eq!(config.worker, 4); // Should fall back to default worker count: 4
    }

    /// Test case for passing a valid worker value.
    #[test]
    fn test_valid_worker() {
        let args = vec!["".to_string(), "--worker".to_string(), "8".to_string()];
        let config = Config::parse_args(args);

        assert_eq!(config.worker, 8);
    }
}
