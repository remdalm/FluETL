use crate::domain::DomainError;

#[derive(Debug, PartialEq, Clone)]
pub struct Locale(String);

impl Locale {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Locale {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('_').collect();

        if parts.len() == 2 {
            let language = parts[0];
            let region = parts[1];

            if language.len() == 2 && region.len() == 2 {
                return Ok(Locale(String::from(value)));
            }
        }

        Err(DomainError::ValidationError(
            "Locale must be in the format of xx_XX".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_locale() {
        // Test valid locale string
        let locale_str = "en_US";
        let result = Locale::try_from(locale_str);
        assert!(result.is_ok());

        // Check if the parsed locale value is correct
        let locale = result.unwrap();
        assert_eq!(locale.as_str(), locale_str);
    }

    #[test]
    fn test_invalid_locale() {
        // Test invalid locale strings
        let invalid_locales = vec!["en", "en_US_", "en_USA", "en-US"];
        for locale_str in invalid_locales {
            let result = Locale::try_from(locale_str);
            assert!(result.is_err());

            // Check if the correct error variant is returned
            let error = result.unwrap_err();
            match error {
                DomainError::ValidationError(msg) => {
                    assert_eq!(msg, "Locale must be in the format of xx_XX".to_string());
                }
                _ => panic!("Expected ValidationError"),
            }
        }
    }
}
