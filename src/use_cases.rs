use std::fmt::Debug;

use crate::{
    domain::DomainError,
    infrastructure::{
        csv_reader::CsvType,
        database::{
            batch::Batch,
            connection::{HasConnection, HasLegacyStagingConnection, HasTargetConnection},
        },
        InfrastructureError,
    },
    interface_adapters::mappers::MappingError,
};

pub(crate) mod helpers;
pub(crate) mod import_delivery_slip;
pub(crate) mod import_invoice;
pub(crate) mod import_mapping_client;
pub(crate) mod import_order;
pub(crate) mod import_order_line;

pub use import_delivery_slip::ImportDeliverySlipUseCase;
pub use import_invoice::ImportInvoiceUseCase;
pub use import_mapping_client::ImportMappingClientUseCase;
pub use import_order::ImportOrderUseCase;
pub use import_order_line::ImportOrderLineUseCase;

#[derive(Debug)]
pub enum UseCaseError {
    Domain(DomainError),
    Infrastructure(InfrastructureError),
    Mapping(MappingError),
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
