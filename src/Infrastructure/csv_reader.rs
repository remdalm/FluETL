use csv::ReaderBuilder;
use serde::Deserialize;
use std::env::{self, VarError};
use std::fs::File;
use std::path::{Path, PathBuf};

use super::InfrastructureError;

pub(crate) mod delivery_slip;
pub(crate) mod invoice;
pub(crate) mod mapping_client;
pub(crate) mod order;
pub(crate) mod order_line;

#[allow(dead_code)]
pub enum CsvType {
    DeliverySlip,
    Invoice,
    Order,
    OrderLine,
    Test(PathBuf),
}

impl CsvType {
    fn get_path(&self) -> Result<String, VarError> {
        match self {
            CsvType::DeliverySlip => env::var("DELIVERY_SLIPS_CSV_PATH"),
            CsvType::Invoice => env::var("INVOICES_CSV_PATH"),
            CsvType::Order => env::var("ORDERS_CSV_PATH"),
            CsvType::OrderLine => env::var("ORDER_LINES_CSV_PATH"),
            CsvType::Test(path) => Ok(path
                .to_str()
                .expect("CsvType::Test cannot be cast into &str")
                .to_string()),
        }
    }
}

#[derive(Debug)]
pub enum CsvError {
    IOError(std::io::Error),
    CsvParseError(csv::Error),
}

pub trait CsvDTO {}

#[derive(Debug)]
pub struct CsvFileReader {
    file_path: PathBuf,
    delimiter: u8,
}

impl Default for CsvFileReader {
    fn default() -> Self {
        Self {
            file_path: Default::default(),
            delimiter: b';',
        }
    }
}

impl CsvFileReader {
    fn new(file_path: PathBuf, delimiter: u8) -> Self {
        CsvFileReader {
            file_path,
            delimiter,
        }
    }

    pub fn read<T>(&self) -> Result<Vec<T>, CsvError>
    where
        T: CsvDTO + for<'a> Deserialize<'a>,
    {
        let mut csv_dtos = Vec::new();

        let file = File::open(self.file_path.as_path()).map_err(CsvError::IOError)?;
        let mut rdr = ReaderBuilder::new()
            .delimiter(self.delimiter)
            .from_reader(file);

        for result in rdr.deserialize::<T>() {
            let csv_dto = result.map_err(CsvError::CsvParseError)?;
            csv_dtos.push(csv_dto);
        }

        Ok(csv_dtos)
    }
}

pub fn make_csv_file_reader(
    csv_type: CsvType,
    delimiter: u8,
) -> Result<CsvFileReader, InfrastructureError> {
    let file_path = csv_type
        .get_path()
        .map_err(InfrastructureError::EnvVarError)?;

    if !Path::new(&file_path).exists() {
        return Err(InfrastructureError::CSVFileNotFound(file_path));
    }

    Ok(CsvFileReader::new(PathBuf::from(file_path), delimiter))
}
