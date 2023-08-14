use serde::Deserialize;

use crate::{
    domain::{DomainEntity, DomainError},
    infrastructure::{
        converters::convert_csv_dto_to_domain_entity,
        csv_reader::{make_csv_file_reader, CsvDTO, CsvType},
        InfrastructureError,
    },
};

mod import_orders;

pub trait UseCase {
    fn execute(&self) -> Result<(), UseCaseError>;
}

trait CanReadCsvUseCase<T, D>
where
    T: CsvDTO + for<'a> Deserialize<'a> + Into<Result<D, DomainError>>,
    D: DomainEntity,
{
    fn read_csv(&self, csv_type: CsvType) -> Result<Vec<T>, UseCaseError> {
        let csv_reader = make_csv_file_reader(csv_type, b';')?;

        let csv_data: Vec<T> = csv_reader
            .read()
            .map_err(|err| UseCaseError::InfrastructureError(InfrastructureError::CsvError(err)))?;
        Ok(csv_data)
    }
    fn parse_csv_data(&self, csv_data: Vec<T>) -> Vec<Result<D, DomainError>> {
        convert_csv_dto_to_domain_entity(csv_data)
    }
}

// trait CanPersistIntoDatabaseUseCase<D>
// where
//     D: DomainEntity,
// {
//     fn persist(entity: D) -> Result<(), UseCaseError> {}
// }

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
