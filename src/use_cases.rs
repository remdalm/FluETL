use std::fmt::Debug;

use serde::Deserialize;

use crate::{
    domain::{DomainEntity, DomainError},
    infrastructure::{
        csv_reader::{make_csv_file_reader, CsvDTO, CsvType},
        database::{
            connection::{HasLegacyStagingConnection, HasTargetConnection},
            models::{CanSelectAllModel, CanUpsertModel, Model},
        },
        InfrastructureError,
    },
    interface_adapters::mappers::{
        convert_csv_dto_to_domain_entity, convert_domain_entity_to_model, MapperError,
        ModelToEntityParser,
    },
};

pub(crate) mod import_mapping_client;
pub(crate) mod import_orders;

pub use import_mapping_client::ImportMappingClientUseCase;
pub use import_orders::ImportOrderUseCase;

pub trait ImportModelUseCase<M1, DE, M2>:
    CanReadAllModelUseCase<ModelImpl = M1>
    + CanPersistIntoDatabaseUseCase<DE, M2>
    + ModelToEntityParser<M1, DE>
where
    M1: Model + Into<Result<DE, MapperError>> + Debug,
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

pub trait ImportCsvUseCase<CSV, DE, M>
where
    CSV: CsvDTO + for<'a> Deserialize<'a> + Into<Result<DE, MapperError>> + Debug,
    DE: DomainEntity + Into<M>,
    M: CanUpsertModel,
{
    type ManagerImpl: UseCaseImportManager<CSV, DE, M>
        + CanReadCsvUseCase<CSV, DE>
        + CanPersistIntoDatabaseUseCase<DE, M>;

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

pub trait UseCaseImportManager<T, DE, M>
where
    T: Into<Result<DE, MapperError>>,
    DE: DomainEntity + Into<M>,
    M: Model,
{
}

// pub trait UseCaseCsvImportManager<T, DE, M>:
//     UseCaseImportManager<T, DE, M> + CanReadCsvUseCase<T, DE> + CanPersistIntoDatabaseUseCase<DE, M>
// where
//     T: CsvDTO + for<'a> Deserialize<'a> + Into<Result<DE, DomainError>>,
//     DE: DomainEntity + Into<M>,
//     M: Model,
// {
// }

pub trait CanReadCsvUseCase<T, DE>
where
    T: CsvDTO + for<'a> Deserialize<'a> + Into<Result<DE, MapperError>>,
    DE: DomainEntity,
{
    fn read(&self, csv_type: CsvType) -> Result<Vec<T>, UseCaseError> {
        let csv_reader = make_csv_file_reader(csv_type, b';')?;

        let csv_data: Vec<T> = csv_reader
            .read()
            .map_err(|err| UseCaseError::InfrastructureError(InfrastructureError::CsvError(err)))?;
        Ok(csv_data)
    }
    fn parse(&self, csv_data: Vec<T>) -> Vec<Result<DE, MapperError>> {
        convert_csv_dto_to_domain_entity(csv_data)
    }
}

pub trait CanReadAllModelUseCase: HasLegacyStagingConnection {
    type ModelImpl: CanSelectAllModel + Model + Debug;
    fn read_all(&self) -> Result<Vec<Self::ModelImpl>, UseCaseError> {
        let mut connection = self.get_pooled_connection();
        let data = Self::ModelImpl::select_all(&mut connection).map_err(|err| {
            UseCaseError::InfrastructureError(InfrastructureError::DatabaseError(err))
        })?;
        Ok(data)
    }

    // fn concrete_model(&self) -> Self::ModelImpl;
}

pub trait CanPersistIntoDatabaseUseCase<DE, M>: HasTargetConnection
where
    DE: DomainEntity + Into<M>,
    M: CanUpsertModel,
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
}

#[derive(Debug)]
pub enum UseCaseError {
    DomainError(DomainError),
    InfrastructureError(InfrastructureError),
    MapperError(MapperError),
}

impl From<MapperError> for UseCaseError {
    fn from(error: MapperError) -> Self {
        UseCaseError::MapperError(error)
    }
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
