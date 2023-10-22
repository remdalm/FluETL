use crate::{
    domain::DomainEntity,
    infrastructure::{
        csv_reader::{make_csv_file_reader, CsvDTO, CsvType},
        database::models::CanUpsertModel,
        InfrastructureError,
    },
    interface_adapters::mappers::CSVToEntityParser,
    use_cases::UseCaseError,
};
use log::debug;
use serde::Deserialize;
use std::fmt::Debug;

use super::model::CanPersistIntoDatabaseUseCase;

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
