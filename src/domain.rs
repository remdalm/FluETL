use std::{error::Error, fmt};

pub(crate) mod delivery_slip;
pub(crate) mod dto;
pub(crate) mod invoice;
pub(crate) mod language;
pub(crate) mod mapping_client;
pub(crate) mod new_type;
pub(crate) mod order;
pub(crate) mod order_line;
pub(crate) mod product;
pub(crate) mod vo;

pub trait DomainEntity {}

#[derive(Debug, PartialEq)]
pub enum DomainError {
    ValidationError(String),
    ParsingError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DomainError {}
