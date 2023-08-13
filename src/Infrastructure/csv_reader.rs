use csv::ReaderBuilder;
use serde::Deserialize;
use std::env::{self, VarError};
use std::fs::File;
use std::path::{Path, PathBuf};

use super::InfrastructureError;

pub enum CsvType {
    Orders,
    MappingClient,
    Test(PathBuf),
}

impl CsvType {
    fn get_path(&self) -> Result<String, VarError> {
        match self {
            CsvType::Orders => env::var("ORDERS_FILE_PATH"),
            CsvType::MappingClient => env::var("MAPPING_CLIENT_FILE_PATH"),
            CsvType::Test(path) => Ok(path
                .to_str()
                .expect("CsvType::Test cannot be cast into &str")
                .to_string()),
        }
    }
}

#[derive(Debug)]
pub enum CsvReaderError {
    IOError(std::io::Error),
    CsvParseError(csv::Error),
}

trait CsvDTO {}

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

    fn read<T>(&self) -> Result<Vec<T>, CsvReaderError>
    where
        T: CsvDTO + for<'a> Deserialize<'a> + PartialEq + Clone,
    {
        let mut csv_dtos = Vec::new();

        let file = File::open(self.file_path.as_path()).map_err(CsvReaderError::IOError)?;
        let mut rdr = ReaderBuilder::new()
            .delimiter(self.delimiter)
            .from_reader(file);

        for result in rdr.deserialize::<T>() {
            let csv_dto = result.map_err(CsvReaderError::CsvParseError)?;
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
        .map_err(|err| InfrastructureError::VarError(err))?;

    if !Path::new(&file_path).exists() {
        return Err(InfrastructureError::FileNotFound(file_path));
    }

    Ok(CsvFileReader::new(PathBuf::from(file_path), delimiter))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fixtures::{csv_order_dto_fixtures, mapping_client_fixtures};
    use crate::tests::fixtures::{MAPPING_CLIENT_CSV, ORDER_CSV};
    use std::io::Write;
    use tempfile::NamedTempFile;

    const DELIMITER: u8 = b';';

    // Helper function to create a temporary CSV file for testing
    fn create_temp_csv(content: &str) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp CSV file");
        temp_file
            .write_all(content.as_bytes())
            .expect("Failed to write to temp CSV file");
        temp_file
    }

    #[test]
    fn test_read_csv_with_same_nb_of_field_that_struct() {
        let temp_csv = create_temp_csv(ORDER_CSV);
        let csv_reader =
            make_csv_file_reader(CsvType::Test(temp_csv.path().to_path_buf()), DELIMITER)
                .expect("Failed to create csv_reader");
        let order_fixture = csv_order_dto_fixtures();

        // Act
        let result: Result<Vec<CsvOrderDTO>, CsvReaderError> = csv_reader.read();

        //Assert
        assert!(result.is_ok(), "Expected successful read_orders");
        let orders = result.unwrap();
        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0], order_fixture[0]);
        assert_eq!(orders[1], order_fixture[1]);
    }

    #[test]
    fn test_read_csv_with_different_nb_of_field_that_struct() {
        let temp_csv = create_temp_csv(MAPPING_CLIENT_CSV);
        let csv_reader =
            make_csv_file_reader(CsvType::Test(temp_csv.path().to_path_buf()), DELIMITER)
                .expect("Failed to create csv_reader");
        let mapping_client_fixtures = mapping_client_fixtures();

        // Act
        let result: Result<Vec<CsvMappingClientDTO>, CsvReaderError> = csv_reader.read();

        //Assert
        assert!(result.is_ok(), "Expected successful read_orders");
        let mapping_client_dtos = result.unwrap();
        assert_eq!(mapping_client_dtos.len(), 2);
        assert_eq!(mapping_client_dtos[0], mapping_client_fixtures[0]);
        assert_eq!(mapping_client_dtos[1], mapping_client_fixtures[1]);
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
        if let InfrastructureError::FileNotFound(file) = error_variant {
            assert_eq!(file, invalid_file_path);
        } else {
            assert!(false, "Expected InfrastructureError::FileNotFound");
        }
    }
}
