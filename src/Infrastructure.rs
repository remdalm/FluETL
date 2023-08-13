mod converters;
pub mod csv_reader;
pub mod database;
mod db_writer;
pub mod environment;

#[derive(Debug)]
pub enum InfrastructureError {
    FileNotFound(String),
    VarError(std::env::VarError),
}
