use std::{error::Error, fmt};

pub(crate) mod csv_reader;
pub(crate) mod data_source;
pub mod database;
pub(crate) mod logger;
pub(crate) mod repository;

#[derive(Debug)]
pub enum InfrastructureError {
    CsvError(csv_reader::CsvError),
    CSVFileNotFound(String),
    EnvVarError(std::env::VarError),
    DatabaseError(diesel::result::Error),
    InconsistentDataError(String),
    NotImplementedError(String),
    LookupError(String),
}

impl fmt::Display for InfrastructureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InfrastructureError {}
