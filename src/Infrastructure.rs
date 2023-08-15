pub(crate) mod converters;
pub(crate) mod csv_reader;
pub(crate) mod database;
pub(crate) mod environment;
pub(crate) mod logger;

#[derive(Debug)]
pub enum InfrastructureError {
    CsvError(csv_reader::CsvError),
    CSVFileNotFound(String),
    EnvVarError(std::env::VarError),
    DatabaseError(diesel::result::Error),
}
