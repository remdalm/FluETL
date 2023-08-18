pub(crate) mod mapping_client;
pub(crate) mod order;

use crate::domain::{DomainEntity, DomainError};
use crate::infrastructure::database::models::Model;
use crate::infrastructure::InfrastructureError;

#[derive(Debug)]
pub enum MappingError {
    InfrastructureError(InfrastructureError),
    DomainError(DomainError),
}

impl From<InfrastructureError> for MappingError {
    fn from(e: InfrastructureError) -> Self {
        MappingError::InfrastructureError(e)
    }
}

impl From<DomainError> for MappingError {
    fn from(e: DomainError) -> Self {
        MappingError::DomainError(e)
    }
}

pub fn convert_csv_dto_to_domain_entity<CSV, DE>(dtos: Vec<CSV>) -> Vec<Result<DE, MappingError>>
where
    CSV: Into<Result<DE, MappingError>>,
    DE: DomainEntity,
{
    dtos.into_iter().map(|dto| dto.into()).collect()
}

pub fn convert_domain_entity_to_model<DE, M>(d: Vec<DE>) -> Vec<M>
where
    DE: DomainEntity + Into<M>,
    M: Model,
{
    d.into_iter().map(|de| de.into()).collect()
}

pub trait ModelToEntityParser<M, DE>
where
    M: Model + Into<Result<DE, MappingError>>,
    DE: DomainEntity,
{
    fn parse_all(&self, models: Vec<M>) -> Vec<Result<DE, MappingError>> {
        models.into_iter().map(|de| de.into()).collect()
    }
}

// fn convert_model_to_domain_entity<M, DE>(model_dtos: Vec<M>) -> Vec<Result<DE, MappingError>>
// where
//     M: Model + Into<Result<DE, MappingError>>,
//     DE: DomainEntity,
// {
//     model_dtos.into_iter().map(|de| de.into()).collect()
// }
