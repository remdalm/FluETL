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
            .replace("%", "")
            .parse::<f32>()
            .map_err(|err| {
                DomainError::ParsingError(
                    err.to_string() + format!(": completion => {}", value).as_str(),
                )
            })
            .and_then(|number| Ok(number.round() as u32))?;
        Ok(Self(completion))
    }
}

impl From<u32> for Completion {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
