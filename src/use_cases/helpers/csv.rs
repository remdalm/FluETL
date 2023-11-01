use crate::{
    domain::DomainEntity,
    infrastructure::{
        csv_reader::CsvDTO, data_source::CanReadCSVDataSource, database::models::CanUpsertModel,
    },
    interface_adapters::mappers::CsvEntityParser,
    use_cases::UseCaseError,
};
use log::debug;
use serde::Deserialize;
use std::fmt::Debug;

use super::model::CanPersistIntoDatabaseUseCase;

pub(crate) trait ImportFromSingleEntityBasedCsvUseCase<CSV, DE, M>:
    CanReadCSVDataSource<CSV> + CsvEntityParser<CSV, DE> + CanPersistIntoDatabaseUseCase<DE, M>
where
    CSV: CsvDTO + for<'a> Deserialize<'a> + Debug,
    DE: DomainEntity + Into<M>,
    M: CanUpsertModel,
{
    fn execute(&self) -> Option<Vec<UseCaseError>> {
        let data = self.find_all();
        if data.is_err() {
            return Some(vec![data.unwrap_err().into()]);
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
}
