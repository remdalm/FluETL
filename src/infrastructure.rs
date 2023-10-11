pub(crate) mod csv_reader;
pub mod database;
pub(crate) mod logger;

#[derive(Debug)]
pub enum InfrastructureError {
    CsvError(csv_reader::CsvError),
    CSVFileNotFound(String),
    EnvVarError(std::env::VarError),
    DatabaseError(diesel::result::Error),
    InconsistentDataError(String),
}
