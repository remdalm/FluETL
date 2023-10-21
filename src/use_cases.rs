use std::fmt::Debug;

use log::debug;
use serde::Deserialize;

use crate::{
    domain::{language::Language, DomainEntity, DomainError},
    infrastructure::{
        csv_reader::{make_csv_file_reader, CsvDTO, CsvType},
        database::{
            batch::Batch,
            connection::{HasConnection, HasLegacyStagingConnection, HasTargetConnection},
            models::{language::LanguageModel, CanSelectAllModel, CanUpsertModel, Model},
        },
        InfrastructureError,
    },
    interface_adapters::mappers::{
        convert_domain_entity_to_model, CSVToEntityParser, MappingError, ModelToEntityParser,
    },
};

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

pub(crate) trait ImportCsvUseCase<CSV, DE, M>:
    CanReadCsvUseCase<CSV> + CSVToEntityParser<CSV, DE> + CanPersistIntoDatabaseUseCase<DE, M>
where
    CSV: CsvDTO + for<'a> Deserialize<'a> + Debug,
    DE: DomainEntity + Into<M>,
    M: CanUpsertModel,
{
    fn execute(&self) -> Option<Vec<UseCaseError>> {
        let data = self.read(self.get_csv_type());
        if data.is_err() {
            return Some(vec![data.unwrap_err()]);
        }
        let csv_row = data.unwrap();
        debug!("Extract {} Csv Rows", csv_row.len());
        let dirty_entities = self.parse_all(csv_row);

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

        Option::from(errors).filter(|e| !e.is_empty())
    }
    fn get_csv_type(&self) -> CsvType;
}

pub trait CanReadCsvUseCase<T>
where
    T: CsvDTO + for<'a> Deserialize<'a>,
{
    fn read(&self, csv_type: CsvType) -> Result<Vec<T>, UseCaseError> {
        let csv_reader = make_csv_file_reader(csv_type, b';')?;

        let csv_data: Vec<T> = csv_reader
            .read()
            .map_err(|err| UseCaseError::Infrastructure(InfrastructureError::CsvError(err)))?;
        Ok(csv_data)
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

pub(crate) trait CanFetchLanguages {
    fn fetch_languages() -> Result<Vec<Language>, UseCaseError> {
        LanguageModel::select_all(&mut HasLegacyStagingConnection::get_pooled_connection())
            .map_err(|e| UseCaseError::Infrastructure(InfrastructureError::DatabaseError(e)))
            .and_then(|models| {
                models
                    .into_iter()
                    .map(|m| m.try_into().map_err(UseCaseError::Mapping))
                    .collect()
            })
    }
}

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
