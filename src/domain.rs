mod order;

mod order_line;

pub use order::Order;
pub use order_line::OrderLine;

#[derive(Debug)]
pub enum DomainError {
    ValidationError(String),
    ParsingError(String),
}
