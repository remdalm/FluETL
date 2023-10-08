use std::fmt;

use crate::domain::DomainError;

#[derive(Debug, PartialEq, Clone)]
pub struct FileName {
    name: String,
    extension: String,
}

impl TryFrom<String> for FileName {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('.').collect();
        if parts.len() == 2
            && !parts[0].is_empty()
            && !parts[0]
                .chars()
                .any(|c| !(c.is_alphanumeric() || c == '-' || c == '_'))
            && !parts[1].is_empty()
        {
            Ok(FileName {
                name: String::from(parts[0]),
                extension: String::from(parts[1]),
            })
        } else {
            Err(DomainError::ValidationError(format!(
                "Invalid file name: {}",
                value
            )))
        }
    }
}

impl fmt::Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.name, self.extension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_file_name() {
        let file_name = FileName::try_from(String::from("example.txt")).unwrap();
        assert_eq!(file_name.name, "example");
        assert_eq!(file_name.extension, "txt");
    }

    #[test]
    fn test_valid_file_name_with_hyphen() {
        let file_name = FileName::try_from(String::from("example-1.txt")).unwrap();
        assert_eq!(file_name.name, "example-1");
        assert_eq!(file_name.extension, "txt");
    }

    #[test]
    fn test_invalid_no_extension() {
        let result = FileName::try_from(String::from("example"));
        assert_error_result(result, "Invalid file name: example");
    }

    #[test]
    fn test_invalid_empty_name() {
        let result = FileName::try_from(String::from(".txt"));
        assert_error_result(result, "Invalid file name: .txt");
    }

    #[test]
    fn test_invalid_empty_extension() {
        let result = FileName::try_from(String::from("example."));
        assert_error_result(result, "Invalid file name: example.");
    }

    #[test]
    fn test_invalid_name_with_space() {
        let result = FileName::try_from(String::from("example -1.pdf"));
        assert_error_result(result, "Invalid file name: example -1.pdf");
    }

    #[test]
    fn test_invalid_no_alphanumeric_name() {
        let result = FileName::try_from(String::from("example&.pdf"));
        assert_error_result(result, "Invalid file name: example&.pdf");
    }

    #[test]
    fn test_invalid_empty_name_and_extension() {
        let result = FileName::try_from(String::from("."));
        assert_error_result(result, "Invalid file name: .");
    }

    #[test]
    fn test_invalid_empty_file_name() {
        let result = FileName::try_from(String::from(""));
        assert_error_result(result, "Invalid file name: ");
    }

    #[test]
    fn test_invalid_file_name_multiple_dots() {
        let result = FileName::try_from(String::from("example.txt.pdf"));
        assert_error_result(result, "Invalid file name: example.txt.pdf");
    }

    fn assert_error_result(result: Result<FileName, DomainError>, expected_error_msg: &str) {
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ValidationError(String::from(expected_error_msg))
        );
    }

    #[test]
    fn test_display() {
        let file_name = FileName::try_from(String::from("example.txt")).unwrap();
        assert_eq!(file_name.to_string(), "example.txt");
    }
}
