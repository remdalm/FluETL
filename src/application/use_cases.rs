use std::fmt::Debug;

use serde::Deserialize;

use crate::{
    benches::database_connection::DbConnection,
    domain::{DomainEntity, DomainError},
    infrastructure::{
        converters::{convert_csv_dto_to_domain_entity, convert_domain_entity_to_model},
        csv_reader::{make_csv_file_reader, CsvDTO, CsvType},
        database::{connection::get_pooled_connection, models::Model},
        InfrastructureError,
    },
};

pub(crate) mod import_orders;

pub trait UseCase<T, DE, M>
where
    T: CsvDTO + for<'a> Deserialize<'a> + Into<Result<DE, DomainError>> + Debug,
    DE: DomainEntity + Into<M>,
    M: Model,
{
    type ManagerImpl: UseCaseManager<T, DE, M>;
    fn execute(&self) -> Option<Vec<UseCaseError>> {
        let manager = self.concrete_manager();
        let data = manager.read(self.get_csv_type());
        if data.is_err() {
            return Some(vec![data.unwrap_err()]);
        }
        let dirty_entities = manager.parse(data.unwrap());

        let mut domain_errors = vec![];
        let entities: Vec<DE> = dirty_entities
            .into_iter()
            .filter_map(|entity| entity.map_err(|e| domain_errors.push(e)).ok())
            .collect();

        let database_errors = manager.persist(entities);

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
    fn get_csv_type(&self) -> CsvType;
    fn concrete_manager(&self) -> Self::ManagerImpl;
}

pub trait UseCaseManager<T, DE, M>:
    CanReadCsvUseCase<T, DE> + CanPersistIntoDatabaseUseCase<DE, M>
where
    T: CsvDTO + for<'a> Deserialize<'a> + Into<Result<DE, DomainError>>,
    DE: DomainEntity + Into<M>,
    M: Model,
{
}

pub trait CanReadCsvUseCase<T, DE>
where
    T: CsvDTO + for<'a> Deserialize<'a> + Into<Result<DE, DomainError>>,
    DE: DomainEntity,
{
    fn read(&self, csv_type: CsvType) -> Result<Vec<T>, UseCaseError> {
        let csv_reader = make_csv_file_reader(csv_type, b';')?;

        let csv_data: Vec<T> = csv_reader
            .read()
            .map_err(|err| UseCaseError::InfrastructureError(InfrastructureError::CsvError(err)))?;
        Ok(csv_data)
    }
    fn parse(&self, csv_data: Vec<T>) -> Vec<Result<DE, DomainError>> {
        convert_csv_dto_to_domain_entity(csv_data)
    }
}

pub trait CanPersistIntoDatabaseUseCase<DE, M>
where
    DE: DomainEntity + Into<M>,
    M: Model,
{
    fn persist(&self, entities: Vec<DE>) -> Option<Vec<InfrastructureError>> {
        let mut errors: Vec<InfrastructureError> = Vec::new();
        let mut connection = self.get_pooled_connection();
        let models: Vec<M> = convert_domain_entity_to_model(entities);

        for model in models {
            let _ = model
                .upsert(&mut connection)
                .map_err(|err| errors.push(InfrastructureError::DatabaseError(err)));
        }
        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    fn get_pooled_connection(&self) -> DbConnection {
        get_pooled_connection()
    }
}

#[derive(Debug)]
pub enum UseCaseError {
    DomainError(DomainError),
    InfrastructureError(InfrastructureError),
}

impl From<DomainError> for UseCaseError {
    fn from(error: DomainError) -> Self {
        UseCaseError::DomainError(error)
    }
}

impl From<diesel::result::Error> for UseCaseError {
    fn from(error: diesel::result::Error) -> Self {
        UseCaseError::InfrastructureError(InfrastructureError::DatabaseError(error))
    }
}

impl From<InfrastructureError> for UseCaseError {
    fn from(error: InfrastructureError) -> Self {
        UseCaseError::InfrastructureError(error)
    }
}
