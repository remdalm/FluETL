pub(crate) mod mapping_client;
pub(crate) mod order;
pub(crate) mod order_line;

pub trait DomainEntity {}

#[derive(Debug)]
pub enum DomainError {
    ValidationError(String),
    ParsingError(String),
}

pub fn convert_string_to_option_string(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
