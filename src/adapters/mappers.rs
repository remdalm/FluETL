pub(crate) mod mapping_client;
pub(crate) mod order;

use crate::domain::{DomainEntity, DomainError};
use crate::infrastructure::database::models::Model;

pub fn convert_csv_dto_to_domain_entity<CSV, DE>(dtos: Vec<CSV>) -> Vec<Result<DE, DomainError>>
where
    CSV: Into<Result<DE, DomainError>>,
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
