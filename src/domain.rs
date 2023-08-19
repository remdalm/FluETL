mod mapping_client;
mod order;
mod order_line;

pub use mapping_client::MappingClient;
pub use order::Order;
pub use order_line::OrderLine;

pub trait DomainEntity {}

#[derive(Debug)]
pub enum DomainError {
    ValidationError(String),
    ParsingError(String),
}

fn convert_string_to_option_string(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
