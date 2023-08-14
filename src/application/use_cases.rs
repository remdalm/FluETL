use serde::Deserialize;

use crate::{
    domain::{DomainEntity, DomainError},
    infrastructure::{
        converters::{convert_csv_dto_to_domain_entity, convert_domain_entity_to_model},
        csv_reader::{make_csv_file_reader, CsvDTO, CsvType},
        database::{connection::get_pooled_connection, models::Model},
        InfrastructureError,
    },
};

mod import_orders;

pub trait UseCase {
    fn execute(&self) -> Result<(), UseCaseError>;
}

trait CanReadCsvUseCase<T, DE>
where
    T: CsvDTO + for<'a> Deserialize<'a> + Into<Result<DE, DomainError>>,
    DE: DomainEntity,
{
    fn read_csv(&self, csv_type: CsvType) -> Result<Vec<T>, UseCaseError> {
        let csv_reader = make_csv_file_reader(csv_type, b';')?;

        let csv_data: Vec<T> = csv_reader
            .read()
            .map_err(|err| UseCaseError::InfrastructureError(InfrastructureError::CsvError(err)))?;
        Ok(csv_data)
    }
    fn parse_csv_data(&self, csv_data: Vec<T>) -> Vec<Result<DE, DomainError>> {
        convert_csv_dto_to_domain_entity(csv_data)
    }
}

trait CanPersistIntoDatabaseUseCase<DE, M>
where
    DE: DomainEntity + Into<M>,
    M: Model,
{
    fn persist(entities: Vec<DE>) -> Option<Vec<InfrastructureError>> {
        let mut errors: Vec<InfrastructureError> = Vec::new();
        let mut connection = get_pooled_connection();
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
