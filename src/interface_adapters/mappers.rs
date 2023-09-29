pub(crate) mod delivery_slip;
pub(crate) mod mapping_client;
pub(crate) mod order;
pub(crate) mod order_line;

use chrono::NaiveDate;

use crate::domain::{DomainEntity, DomainError};
use crate::infrastructure::csv_reader::CsvDTO;
use crate::infrastructure::database::models::Model;
use crate::infrastructure::InfrastructureError;

#[derive(Debug)]
pub enum MappingError {
    InfrastructureError(InfrastructureError),
    DomainError(DomainError),
    ParsingError(String),
    CacheError,
}

pub trait GenericMapperParser<S, D> {
    fn parse_all(&self, sources: Vec<S>) -> Vec<Result<D, MappingError>>
    where
        S: TryInto<D>,
        Vec<Result<D, MappingError>>: FromIterator<Result<D, <S as TryInto<D>>::Error>>,
    {
        sources.into_iter().map(|s| s.try_into()).collect()
    }
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

pub fn convert_domain_entity_to_model<DE, M>(d: Vec<DE>) -> Vec<M>
where
    DE: DomainEntity + Into<M>,
    M: Model,
{
    d.into_iter().map(|de| de.into()).collect()
}

pub trait ToDomainEntityParser<T, DE>
where
    T: TryInto<DE, Error = MappingError>,
    DE: DomainEntity,
{
    fn parse_all(&self, models: Vec<T>) -> Vec<Result<DE, MappingError>> {
        models.into_iter().map(|de| de.try_into()).collect()
    }
}

pub trait CSVToEntityParser<CSV, DE>
where
    CSV: CsvDTO,
    DE: DomainEntity,
{
    fn parse_all(&self, csv_dtos: Vec<CSV>) -> Vec<Result<DE, MappingError>> {
        csv_dtos
            .into_iter()
            .map(|s| self.transform_csv(s))
            .collect()
    }

    fn parse(&self, csv_dto: CSV) -> Result<DE, MappingError> {
        self.transform_csv(csv_dto)
    }

    fn transform_csv(&self, csv: CSV) -> Result<DE, MappingError>;
}

pub trait ModelToEntityParser<M, DE>
where
    M: Model + TryInto<DE, Error = MappingError>,
    DE: DomainEntity,
{
    fn parse_all(&self, models: Vec<M>) -> Vec<Result<DE, MappingError>> {
        models.into_iter().map(|de| de.try_into()).collect()
    }
}

pub fn convert_string_to_option_string(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

pub fn convert_string_to_option_date(
    s: String,
    fmt: &str,
) -> Option<Result<NaiveDate, MappingError>> {
    let s_date = convert_string_to_option_string(s);
    if s_date.is_some() {
        let date =
            NaiveDate::parse_from_str(s_date.as_ref().unwrap().as_str(), fmt).map_err(|err| {
                MappingError::ParsingError(
                    err.to_string() + format!(": date => {}", s_date.unwrap()).as_str(),
                )
            });
        Some(date)
    } else {
        None
    }
}

pub fn parse_string_to_u32(key: &str, value: &str) -> Result<u32, MappingError> {
    value.parse::<u32>().map_err(|e| {
        MappingError::ParsingError(e.to_string() + format!(": {} => {}", key, value).as_str())
    })
}
