use crate::domain::DomainError;

#[derive(Debug, PartialEq, Clone)]
pub struct FilledString(String);

impl FilledString {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.chars().any(|c| c.is_alphanumeric()) {
            Ok(Self(value))
        } else {
            Err(DomainError::ValidationError(error_string(&value)))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn error_string(s: &str) -> String {
    format!(
        "FilledString: The string '{}' should contain at least one alphanumeric characters",
        s
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_filled_string_with_non_empty_alphanumeric_string() {
        // Test creating a FilledString with a non-empty alphanumeric string
        let filled_string = FilledString::new("aBc 123".to_string());
        assert!(filled_string.is_ok());
        assert_eq!(filled_string.unwrap().as_str(), "aBc 123");
    }

    #[test]
    fn test_new_filled_string_with_empty_string() {
        // Test creating a FilledString with an empty string
        let tested_value: &str = "";
        let filled_string = FilledString::new(tested_value.to_string());
        assert!(filled_string.is_err());
        assert_eq!(
            filled_string.unwrap_err(),
            DomainError::ValidationError(error_string(tested_value))
        );
    }

    #[test]
    fn test_new_filled_string_with_space() {
        // Test creating a FilledString with a space
        let tested_value: &str = " ";
        let filled_string = FilledString::new(tested_value.to_string());
        assert!(filled_string.is_err());
        assert_eq!(
            filled_string.unwrap_err(),
            DomainError::ValidationError(error_string(tested_value))
        );
    }

    #[test]
    fn test_new_filled_string_with_non_alphanumeric_string() {
        // Test creating a FilledString with a non-alphanumeric string
        let tested_value: &str = "!@#$%^&*()";
        let filled_string = FilledString::new(tested_value.to_string());
        assert!(filled_string.is_err());
        assert_eq!(
            filled_string.unwrap_err(),
            DomainError::ValidationError(error_string(tested_value))
        );
    }
}
