pub(crate) mod delivery_slip;
pub(crate) mod dto;
pub(crate) mod invoice;
pub(crate) mod mapping_client;
pub(crate) mod new_type;
pub(crate) mod order;
pub(crate) mod order_line;
pub(crate) mod vo;

pub trait DomainEntity {}

#[derive(Debug, PartialEq)]
pub enum DomainError {
    ValidationError(String),
    ParsingError(String),
}
