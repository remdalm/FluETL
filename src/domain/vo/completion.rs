use crate::domain::DomainError;

#[derive(Debug, PartialEq, Clone)]
pub struct Completion(u32);

impl Completion {
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl TryFrom<String> for Completion {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let completion = value
            .replace('%', "")
            .parse::<f32>()
            .map_err(|err| {
                DomainError::ParsingError(
                    err.to_string() + format!(": completion => {}", value).as_str(),
                )
            })
            .map(|number| number.round() as u32)?;
        Ok(Self(completion))
    }
}

impl From<u32> for Completion {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_as_u32() {
        let completion = Completion::from(50);
        assert_eq!(completion.as_u32(), 50);
    }

    #[test]
    fn test_completion_try_from() {
        let completion = Completion::try_from("50%".to_string());
        assert_eq!(completion.unwrap(), Completion::from(50));
    }

    #[test]
    fn test_completion_try_from_with_invalid_string() {
        let completion = Completion::try_from("invalid".to_string());
        assert!(completion.is_err());
    }

    #[test]
    fn test_completion_from() {
        let completion = Completion::from(50);
        assert_eq!(completion, Completion(50));
    }
}
