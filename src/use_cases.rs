use std::{error::Error, fmt::Debug};

use crate::{
    domain::DomainError, infrastructure::InfrastructureError,
    interface_adapters::mappers::MappingError,
};

pub(crate) mod clear_product;
pub(crate) mod helpers;
pub(crate) mod import_delivery_slip;
pub(crate) mod import_invoice;
pub(crate) mod import_mapping_client;
pub(crate) mod import_order;
pub(crate) mod import_order_line;
pub(crate) mod import_product;
pub trait ExecutableUseCase {
    fn execute(&self) -> Option<Vec<UseCaseError>>;
}

#[derive(Debug)]
pub enum UseCaseError {
    Domain(DomainError),
    Infrastructure(InfrastructureError),
    Mapping(MappingError),
    Unknown(Box<dyn Error>),
}

impl From<MappingError> for UseCaseError {
    fn from(error: MappingError) -> Self {
        UseCaseError::Mapping(error)
    }
}

impl From<DomainError> for UseCaseError {
    fn from(error: DomainError) -> Self {
        UseCaseError::Domain(error)
    }
}

impl From<diesel::result::Error> for UseCaseError {
    fn from(error: diesel::result::Error) -> Self {
        UseCaseError::Infrastructure(InfrastructureError::DatabaseError(error))
    }
}

impl From<InfrastructureError> for UseCaseError {
    fn from(error: InfrastructureError) -> Self {
        UseCaseError::Infrastructure(error)
    }
}

impl From<Box<dyn Error>> for UseCaseError {
    fn from(error: Box<dyn Error>) -> Self {
        match error {
            e if e.is::<DomainError>() => {
                UseCaseError::Domain(*e.downcast::<DomainError>().unwrap())
            }
            e if e.is::<InfrastructureError>() => {
                UseCaseError::Infrastructure(*e.downcast::<InfrastructureError>().unwrap())
            }
            e if e.is::<MappingError>() => {
                UseCaseError::Mapping(*e.downcast::<MappingError>().unwrap())
            }
            _ => UseCaseError::Unknown(error),
        }
    }
}
