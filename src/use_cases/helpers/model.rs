use log::debug;
use std::fmt::Debug;

use crate::{
    domain::DomainEntity,
    infrastructure::{
        database::{
            batch::Batch,
            connection::HasConnection,
            models::{CanSelectAllModel, CanUpsertModel, Model},
        },
        InfrastructureError,
    },
    interface_adapters::mappers::{
        convert_domain_entity_to_model, MappingError, ModelToEntityParser,
    },
    use_cases::UseCaseError,
};

pub(crate) trait ImportModelUseCase<M1, DE, M2>:
    CanReadAllModelUseCase<ModelImpl = M1>
    + CanPersistIntoDatabaseUseCase<DE, M2>
    + ModelToEntityParser<M1, DE>
where
    M1: Model + TryInto<DE, Error = MappingError> + Debug,
    DE: DomainEntity + Into<M2>,
    M2: CanUpsertModel,
{
    fn execute(&self) -> Option<Vec<UseCaseError>> where {
        let data = self.read_all();
        if data.is_err() {
            return Some(vec![data.unwrap_err()]);
        }
        let dirty_entities = self.parse_all(data.unwrap());

        let mut domain_errors = vec![];
        let entities: Vec<DE> = dirty_entities
            .into_iter()
            .filter_map(|entity| entity.map_err(|e| domain_errors.push(e)).ok())
            .collect();

        let database_errors = self.persist(entities);

        let mut errors: Vec<UseCaseError> = domain_errors.into_iter().map(|e| e.into()).collect();
        errors.append(
            &mut database_errors
                .unwrap_or(vec![])
                .into_iter()
                .map(|e| e.into())
                .collect(),
        );

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }
}

pub(crate) trait CanReadAllModelUseCase {
    type ModelImpl: CanSelectAllModel + Model + Debug;
    type DbConnection: HasConnection;
    fn read_all(&self) -> Result<Vec<Self::ModelImpl>, UseCaseError> {
        let mut connection = Self::DbConnection::get_pooled_connection();
        let data = Self::ModelImpl::select_all(&mut connection)
            .map_err(|err| UseCaseError::Infrastructure(InfrastructureError::DatabaseError(err)))?;
        debug!("Found {} Entities", data.len());
        Ok(data)
    }
}

pub(crate) trait CanPersistIntoDatabaseUseCase<DE, M>
where
    DE: DomainEntity + Into<M>,
    M: CanUpsertModel,
{
    type DbConnection: HasConnection;

    fn persist(&self, entities: Vec<DE>) -> Option<Vec<InfrastructureError>> {
        let mut errors: Vec<InfrastructureError> = Vec::new();
        let mut connection = Self::DbConnection::get_pooled_connection();
        let models: Vec<M> = convert_domain_entity_to_model(entities);

        if let Some(batch) = self.set_batch(&models) {
            let batch_errors = batch.run();
            if let Some(batch_errors) = batch_errors {
                errors.extend(
                    batch_errors
                        .into_iter()
                        .map(InfrastructureError::DatabaseError),
                );
            }
        } else {
            for model in models {
                let _ = model
                    .upsert(&mut connection)
                    .map_err(|err| errors.push(InfrastructureError::DatabaseError(err)));
            }
        }

        Option::from(errors).filter(|e| !e.is_empty())
    }

    fn set_batch<'a>(&'a self, _models: &'a [M]) -> Option<Batch<M>> {
        None
    }
}
