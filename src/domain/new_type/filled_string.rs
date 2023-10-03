use crate::domain::DomainError;

const ERROR_STRING: &str = "The field should contain at least one alphanumeric characters";

#[derive(Debug, PartialEq, Clone)]
pub struct FilledString(String);

impl FilledString {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.chars().any(|c| c.is_alphanumeric()) {
            Ok(Self(value))
        } else {
            Err(DomainError::ValidationError(ERROR_STRING.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
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
        let filled_string = FilledString::new("".to_string());
        assert!(filled_string.is_err());
        assert_eq!(
            filled_string.unwrap_err(),
            DomainError::ValidationError(ERROR_STRING.to_string())
        );
    }

    #[test]
    fn test_new_filled_string_with_space() {
        // Test creating a FilledString with a space
        let filled_string = FilledString::new(" ".to_string());
        assert!(filled_string.is_err());
        assert_eq!(
            filled_string.unwrap_err(),
            DomainError::ValidationError(ERROR_STRING.to_string())
        );
    }

    #[test]
    fn test_new_filled_string_with_non_alphanumeric_string() {
        // Test creating a FilledString with a non-alphanumeric string
        let filled_string = FilledString::new("!@#$%^&*()".to_string());
        assert!(filled_string.is_err());
        assert_eq!(
            filled_string.unwrap_err(),
            DomainError::ValidationError(ERROR_STRING.to_string())
        );
    }
}
