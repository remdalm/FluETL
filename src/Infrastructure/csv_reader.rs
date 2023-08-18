use csv::ReaderBuilder;
use serde::Deserialize;
use std::env::{self, VarError};
use std::fs::File;
use std::path::{Path, PathBuf};

use super::InfrastructureError;

#[allow(dead_code)]
pub enum CsvType {
    Orders,
    Test(PathBuf),
}

impl CsvType {
    fn get_path(&self) -> Result<String, VarError> {
        match self {
            CsvType::Orders => env::var("ORDERS_CSV_PATH"),
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

// DTO Structures
// CsvOrderDTO struct for deserializing CSV data
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CsvOrderDTO {
    pub c_order_id: String,
    pub c_bpartner_id: String,
    pub name: String,
    pub date: String,
    pub order_ref: String,
    pub po_ref: String,
    pub origin: String,
    pub completion: String,
    pub order_status: String,
    pub delivery_status: String,
}

impl CsvDTO for CsvOrderDTO {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CsvMappingClientDTO {
    pub c_bpartner_id: String,
    pub ad_user_id: String,
}

impl CsvDTO for CsvMappingClientDTO {}

impl CsvFileReader {
    fn new(file_path: PathBuf, delimiter: u8) -> Self {
        CsvFileReader {
            file_path: file_path,
            delimiter: delimiter,
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
        .map_err(|err| InfrastructureError::EnvVarError(err))?;

    if !Path::new(&file_path).exists() {
        return Err(InfrastructureError::CSVFileNotFound(file_path));
    }

    Ok(CsvFileReader::new(PathBuf::from(file_path), delimiter))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::fixtures::{
        csv_order_dto_fixtures, ORDER_FLAWLESS_CSV, ORDER_WITH_EMPTY_FIELD_CSV,
        ORDER_WITH_MISSING_DATA_CSV,
    };
    use std::io::Write;
    use tempfile::NamedTempFile;

    pub const DELIMITER: u8 = b';';

    // Helper function to create a temporary CSV file for testing
    pub fn create_temp_csv(content: &str) -> NamedTempFile {
        let mut temp_file: NamedTempFile =
            NamedTempFile::new().expect("Failed to create temp CSV file");
        temp_file
            .write_all(content.as_bytes())
            .expect("Failed to write to temp CSV file");
        temp_file
    }

    #[test]
    fn test_read_flawless_csv() {
        let temp_csv = create_temp_csv(ORDER_FLAWLESS_CSV);
        let csv_reader =
            make_csv_file_reader(CsvType::Test(temp_csv.path().to_path_buf()), DELIMITER)
                .expect("Failed to create csv_reader");
        let order_fixture = csv_order_dto_fixtures();

        // Act
        let result: Result<Vec<CsvOrderDTO>, CsvError> = csv_reader.read();

        //Assert
        assert!(result.is_ok(), "Expected successful read_orders");
        let orders = result.unwrap();
        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0], order_fixture[0]);
        assert_eq!(orders[1], order_fixture[1]);
    }

    #[test]
    fn test_read_csv_with_different_nb_of_field_that_struct() {
        let temp_csv = create_temp_csv(ORDER_WITH_MISSING_DATA_CSV);
        let csv_reader =
            make_csv_file_reader(CsvType::Test(temp_csv.path().to_path_buf()), DELIMITER)
                .expect("Failed to create csv_reader");

        // Act
        let result: Result<Vec<CsvOrderDTO>, CsvError> = csv_reader.read();

        //Assert
        assert!(
            result.is_err_and(|err| match err {
                CsvError::CsvParseError(err) => {
                    match err.kind() {
                        csv::ErrorKind::UnequalLengths {
                            pos: _,
                            expected_len,
                            len,
                        } => {
                            let csv_lengh: u64 = 8;
                            let csv_expexted_lengh: u64 = 10;

                            assert_eq!(expected_len, &csv_expexted_lengh);
                            assert_eq!(len, &csv_lengh);
                            true
                        }
                        _ => false,
                    }
                }
                _ => false,
            }),
            "Expected Error of type csv::ErrorKind::UnequalLengths"
        );
    }

    #[test]
    fn test_read_csv_with_empty_field() {
        let temp_csv = create_temp_csv(ORDER_WITH_EMPTY_FIELD_CSV);
        let csv_reader =
            make_csv_file_reader(CsvType::Test(temp_csv.path().to_path_buf()), DELIMITER)
                .expect("Failed to create csv_reader");
        let order_fixture = csv_order_dto_fixtures();

        // Act
        let result: Result<Vec<CsvOrderDTO>, CsvError> = csv_reader.read();

        //Assert
        assert!(result.is_ok(), "Expected successful read_orders");
        let order_dtos = result.unwrap();
        assert_eq!(order_dtos.len(), 2);
        assert_eq!(order_dtos[1], order_fixture[2]);
    }

    #[test]
    fn test_read_csv_with_invalid_path() {
        // Arrange: Use non-existent paths
        let invalid_file_path = "/invalid/orders.csv";

        // Act
        let result =
            make_csv_file_reader(CsvType::Test(PathBuf::from(invalid_file_path)), DELIMITER);

        // Assert
        let error_variant = result.unwrap_err();
        if let InfrastructureError::CSVFileNotFound(file) = error_variant {
            assert_eq!(file, invalid_file_path);
        } else {
            assert!(false, "Expected InfrastructureError::FileNotFound");
        }
    }
}
