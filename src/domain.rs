pub(crate) mod mapping_client;
pub(crate) mod order;
pub(crate) mod order_line;

pub trait DomainEntity {}

#[derive(Debug)]
pub enum DomainError {
    ValidationError(String),
    ParsingError(String),
}

struct Validator;

impl Validator {
    fn string_is_not_empty(key: &str, value: &str) -> Result<(), DomainError> {
        if value.is_empty() {
            Err(DomainError::ValidationError(format!(
                "The field {} cannot be empty",
                key
            )))
        } else {
            Ok(())
        }
    }
}
