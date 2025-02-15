use katana::templates::{Templates, TemplatesPage};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    trait TemplateExtensions {
        fn new_mock() -> Self;
    }

    impl TemplateExtensions for Templates {
        /// Creates a mock TemplateExtensions instance with predefined template strings
        fn new_mock() -> Self {
            Templates {
                banner: "Welcome, {{username}}!".to_string(),
                error: "Error: {{message}}".to_string(),
                directory: "User: {{username}}, Role: {{role}}".to_string(),
            }
        }
    }

    /// Test that TemplateExtensions are loaded correctly and not empty
    #[test]
    fn test_template_loading() {
        let template_extensions: Templates = TemplateExtensions::new_mock();

        // Ensure template_extensions are loaded correctly
        assert!(
            !template_extensions.banner.is_empty(),
            "Banner template should not be empty"
        );
        assert!(
            !template_extensions.error.is_empty(),
            "Error template should not be empty"
        );
        assert!(
            !template_extensions.directory.is_empty(),
            "Directory template should not be empty"
        );
    }

    /// Test rendering a valid template with parameters
    #[test]
    fn test_rendering_with_valid_template() {
        let template_extensions: Templates = TemplateExtensions::new_mock();
        let mut params = HashMap::new();
        params.insert("username".to_string(), "Alice".to_string());

        let rendered = template_extensions.render(TemplatesPage::BANNER, params);

        assert!(
            rendered.contains("Alice"),
            "Rendered template should contain replaced username"
        );
    }

    /// Test rendering a template with multiple parameters
    #[test]
    fn test_rendering_with_multiple_params() {
        let template_extensions: Templates = TemplateExtensions::new_mock();
        let mut params = HashMap::new();
        params.insert("username".to_string(), "Bob".to_string());
        params.insert("role".to_string(), "Admin".to_string());

        let rendered = template_extensions.render(TemplatesPage::DIRECTORY, params);

        assert!(
            rendered.contains("Bob"),
            "Rendered template should contain 'Bob'"
        );
        assert!(
            rendered.contains("Admin"),
            "Rendered template should contain 'Admin'"
        );
    }

    /// Test rendering a template with missing placeholders
    #[test]
    fn test_rendering_with_missing_placeholder() {
        let template_extensions: Templates = TemplateExtensions::new_mock();
        let params = HashMap::new(); // No params provided

        let rendered = template_extensions.render(TemplatesPage::ERROR, params);

        assert!(
            rendered.contains("{{"),
            "Unreplaced placeholders should remain"
        );
    }

    /// Test rendering a template with empty parameter values
    #[test]
    fn test_rendering_with_empty_params() {
        let template_extensions: Templates = TemplateExtensions::new_mock();
        let mut params = HashMap::new();
        params.insert("username".to_string(), "".to_string());

        let rendered = template_extensions.render(TemplatesPage::BANNER, params);

        assert!(
            rendered.contains("{{username}}"),
            "Empty value should not remove the placeholder"
        );
    }
}
