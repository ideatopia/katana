use katana::logger::{LogLevel, Logger};

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function that writes a log message to an in-memory buffer and returns the output as a String.
    fn capture_log(level: LogLevel, message: &str) -> String {
        let mut buffer = Vec::new();
        Logger::writer(level, message, &mut buffer);
        String::from_utf8(buffer).expect("Output was not valid UTF-8")
    }

    /// Helper function to check that a timestamp (formatted as "YYYY-MM-DD HH:MM:SS")
    /// meets the expected format without using regex.
    fn check_timestamp_format(timestamp: &str) {
        // The expected format is exactly 19 characters long.
        assert_eq!(
            timestamp.len(),
            19,
            "Timestamp length should be 19, got '{}'",
            timestamp
        );

        let chars: Vec<char> = timestamp.chars().collect();
        // Check for expected fixed separators at the proper positions:
        assert_eq!(chars[4], '-', "Expected '-' at index 4, got '{}'", chars[4]);
        assert_eq!(chars[7], '-', "Expected '-' at index 7, got '{}'", chars[7]);
        assert_eq!(
            chars[10], ' ',
            "Expected ' ' at index 10, got '{}'",
            chars[10]
        );
        assert_eq!(
            chars[13], ':',
            "Expected ':' at index 13, got '{}'",
            chars[13]
        );
        assert_eq!(
            chars[16], ':',
            "Expected ':' at index 16, got '{}'",
            chars[16]
        );

        // Check that all other characters are digits.
        for (i, c) in chars.iter().enumerate() {
            if i == 4 || i == 7 || i == 10 || i == 13 || i == 16 {
                continue;
            }
            assert!(
                c.is_ascii_digit(),
                "Character at index {} should be a digit, got '{}'",
                i,
                c
            );
        }
    }

    /// Verify that `Logger::writer` correctly writes log messages to a `Vec<u8>`.
    #[test]
    fn test_writer_function() {
        let message = "Test message for writer function";
        let mut buffer = Vec::new();

        // Call the writer function, using the buffer to capture the output.
        Logger::writer(LogLevel::INFO, message, &mut buffer);

        // Convert the buffer into a string for inspection.
        let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");

        // Ensure that the output contains the timestamp, log level, and message in the correct format.
        assert!(output.starts_with('['), "Output should start with '['");

        // Find the closing bracket for the timestamp.
        let end_timestamp = output
            .find("] ")
            .expect("Missing closing bracket for timestamp");
        // Extract the timestamp (without the leading '[').
        let timestamp = &output[1..end_timestamp];
        check_timestamp_format(timestamp);

        // Check that the log level is correctly included.
        let expected_level = "] [INFO] ";
        assert!(
            output.contains(expected_level),
            "Output should contain '{}', got '{}'",
            expected_level,
            output
        );

        // Verify that the output ends with the log message.
        assert!(
            output.ends_with(message),
            "Output should end with '{}', got '{}'",
            message,
            output
        );
    }

    /// Verify that `Logger::writer` handles an empty message correctly.
    #[test]
    fn test_writer_empty_message() {
        let message = "";
        let mut buffer = Vec::new();

        // Call the writer function, using the buffer to capture the output.
        Logger::writer(LogLevel::DEBUG, message, &mut buffer);

        // Convert the buffer into a string for inspection.
        let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");

        // Ensure that the output contains the timestamp, log level, and an empty message.
        assert!(output.starts_with('['), "Output should start with '['");

        // Find the closing bracket for the timestamp.
        let end_timestamp = output
            .find("] ")
            .expect("Missing closing bracket for timestamp");
        // Extract the timestamp (without the leading '[').
        let timestamp = &output[1..end_timestamp];
        check_timestamp_format(timestamp);

        // Check that the log level is correctly included.
        let expected_level = "] [DEBUG] ";
        assert!(
            output.contains(expected_level),
            "Output should contain '{}', got '{}'",
            expected_level,
            output
        );

        // Verify that the output ends with the empty message.
        assert!(
            output.ends_with(""),
            "Output should end with an empty message"
        );
    }

    /// Verify that `Logger::writer` handles special characters in the message.
    #[test]
    fn test_writer_special_characters() {
        let message = "!@#$%^&*()_+-=[]{}|;':\",.<>/?`~";
        let mut buffer = Vec::new();

        // Call the writer function, using the buffer to capture the output.
        Logger::writer(LogLevel::ERROR, message, &mut buffer);

        // Convert the buffer into a string for inspection.
        let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");

        // Ensure that the output contains the timestamp, log level, and special characters message.
        assert!(output.starts_with('['), "Output should start with '['");

        // Find the closing bracket for the timestamp.
        let end_timestamp = output
            .find("] ")
            .expect("Missing closing bracket for timestamp");
        // Extract the timestamp (without the leading '[').
        let timestamp = &output[1..end_timestamp];
        check_timestamp_format(timestamp);

        // Check that the log level is correctly included.
        let expected_level = "] [ERROR] ";
        assert!(
            output.contains(expected_level),
            "Output should contain '{}', got '{}'",
            expected_level,
            output
        );

        // Verify that the output ends with the special characters message.
        assert!(
            output.ends_with(message),
            "Output should end with special characters message"
        );
    }

    /// Verify that `Logger::writer` handles very long messages.
    #[test]
    fn test_writer_long_message() {
        let message = "A".repeat(10000);
        let mut buffer = Vec::new();

        // Call the writer function, using the buffer to capture the output.
        Logger::writer(LogLevel::WARN, &message, &mut buffer);

        // Convert the buffer into a string for inspection.
        let output = String::from_utf8(buffer).expect("Output was not valid UTF-8");

        // Ensure that the output contains the timestamp, log level, and long message.
        assert!(output.starts_with('['), "Output should start with '['");

        // Find the closing bracket for the timestamp.
        let end_timestamp = output
            .find("] ")
            .expect("Missing closing bracket for timestamp");
        // Extract the timestamp (without the leading '[').
        let timestamp = &output[1..end_timestamp];
        check_timestamp_format(timestamp);

        // Check that the log level is correctly included.
        let expected_level = "] [WARN] ";
        assert!(
            output.contains(expected_level),
            "Output should contain '{}', got '{}'",
            expected_level,
            output
        );

        // Verify that the output ends with the long message.
        assert!(
            output.ends_with(&message),
            "Output should end with long message, got '{}'",
            output
        );
    }

    /// Verify that a DEBUG log message is formatted correctly.
    #[test]
    fn test_debug_log_format() {
        let message = "Test debug message";
        let output = capture_log(LogLevel::DEBUG, message);

        // Expected overall format: "[<timestamp>] [DEBUG] <message>"
        // Check that output starts with '['.
        assert!(output.starts_with('['), "Output should start with '['");

        // Find the closing bracket for the timestamp.
        let end_timestamp = output
            .find("] ")
            .expect("Missing closing bracket for timestamp");
        // Extract the timestamp (without the leading '[').
        let timestamp = &output[1..end_timestamp];
        check_timestamp_format(timestamp);

        // Check that the log level is correctly included.
        let expected_level = "] [DEBUG] ";
        assert!(
            output.contains(expected_level),
            "Output should contain '{}'",
            expected_level
        );

        // Verify that the output ends with the log message.
        assert!(
            output.ends_with(message),
            "Output should end with '{}'",
            message
        );
    }

    /// Verify that an INFO log message is formatted correctly.
    #[test]
    fn test_info_log_format() {
        let message = "Information log";
        let output = capture_log(LogLevel::INFO, message);

        assert!(output.starts_with('['), "Output should start with '['");
        let end_timestamp = output
            .find("] ")
            .expect("Missing closing bracket for timestamp");
        let timestamp = &output[1..end_timestamp];
        check_timestamp_format(timestamp);

        let expected_level = "] [INFO] ";
        assert!(
            output.contains(expected_level),
            "Output should contain '{}'",
            expected_level
        );
        assert!(
            output.ends_with(message),
            "Output should end with '{}'",
            message
        );
    }

    /// Verify that a WARN log message is formatted correctly.
    #[test]
    fn test_warn_log_format() {
        let message = "Warning log";
        let output = capture_log(LogLevel::WARN, message);

        assert!(output.starts_with('['), "Output should start with '['");
        let end_timestamp = output
            .find("] ")
            .expect("Missing closing bracket for timestamp");
        let timestamp = &output[1..end_timestamp];
        check_timestamp_format(timestamp);

        let expected_level = "] [WARN] ";
        assert!(
            output.contains(expected_level),
            "Output should contain '{}'",
            expected_level
        );
        assert!(
            output.ends_with(message),
            "Output should end with '{}'",
            message
        );
    }

    /// Verify that an ERROR log message is formatted correctly.
    #[test]
    fn test_error_log_format() {
        let message = "Error occurred";
        let output = capture_log(LogLevel::ERROR, message);

        assert!(output.starts_with('['), "Output should start with '['");
        let end_timestamp = output
            .find("] ")
            .expect("Missing closing bracket for timestamp");
        let timestamp = &output[1..end_timestamp];
        check_timestamp_format(timestamp);

        let expected_level = "] [ERROR] ";
        assert!(
            output.contains(expected_level),
            "Output should contain '{}'",
            expected_level
        );
        assert!(
            output.ends_with(message),
            "Output should end with '{}'",
            message
        );
    }

    /// Verify that an empty message is handled correctly.
    #[test]
    fn test_empty_message() {
        let message = "";
        let output = capture_log(LogLevel::INFO, message);

        // Expect the output to end with the space after the log level.
        let expected_ending = "] [INFO] ";
        assert!(
            output.ends_with(expected_ending),
            "For an empty message, output should end with '{}', got '{}'",
            expected_ending,
            output
        );

        // Alternatively, extract the part after the log level and confirm it is empty.
        let pos = output
            .rfind(expected_ending)
            .expect("Expected log level part missing");
        let message_part = &output[pos + expected_ending.len()..];
        assert!(
            message_part.is_empty(),
            "Expected empty message, but got '{}'",
            message_part
        );
    }

    /// Verify that messages containing special characters are handled correctly.
    #[test]
    fn test_special_characters_message() {
        let message = "!@#$%^&*()_+-=[]{}|;':\",.<>/?`~";
        let output = capture_log(LogLevel::DEBUG, message);
        assert!(
            output.ends_with(message),
            "Output should end with the special characters message"
        );
    }

    /// Verify that a very long message is handled correctly.
    #[test]
    fn test_long_message() {
        let message = "A".repeat(10000);
        let output = capture_log(LogLevel::ERROR, &message);
        assert!(
            output.ends_with(&message),
            "Output does not end with the long message"
        );
    }

    /// Verify that multiple log messages written to the same buffer are concatenated.
    #[test]
    fn test_multiple_logs_in_buffer() {
        let mut buffer = Vec::new();
        Logger::writer(LogLevel::INFO, "First log", &mut buffer);
        Logger::writer(LogLevel::WARN, "Second log", &mut buffer);
        let output = String::from_utf8(buffer).expect("Invalid UTF-8 output");

        assert!(
            output.contains("] [INFO] First log"),
            "Output does not contain the first log message"
        );
        assert!(
            output.contains("] [WARN] Second log"),
            "Output does not contain the second log message"
        );
    }

    /// Verify that timestamps from consecutive log messages are valid and,
    /// if different, are in non-decreasing (chronological) order.
    #[test]
    fn test_timestamp_monotonicity() {
        let output1 = capture_log(LogLevel::INFO, "First call");
        let output2 = capture_log(LogLevel::INFO, "Second call");

        // Extract timestamp substrings from each output.
        let end1 = output1
            .find("] ")
            .expect("Missing closing bracket in first log");
        let end2 = output2
            .find("] ")
            .expect("Missing closing bracket in second log");
        let ts1 = &output1[1..end1];
        let ts2 = &output2[1..end2];

        check_timestamp_format(ts1);
        check_timestamp_format(ts2);

        // If the two timestamps are not equal, check that they are in chronological order.
        if ts1 != ts2 {
            assert!(
                ts1 <= ts2,
                "Timestamps are not in non-decreasing order: '{}' > '{}'",
                ts1,
                ts2
            );
        }
    }
}
