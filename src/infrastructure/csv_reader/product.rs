use serde::Deserialize;

use crate::infrastructure::data_source::CanReadCSVDataSource;

use super::*;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CsvProductSubstituteDTO {
    pub m_product_id: String,
    pub substitute_id: String,
}

impl CsvDTO for CsvProductSubstituteDTO {}

pub struct ProductCsvDataSourceReader;

impl CanReadCSVDataSource<CsvProductSubstituteDTO> for ProductCsvDataSourceReader {
    fn find_all(&self) -> Result<Vec<CsvProductSubstituteDTO>, InfrastructureError> {
        self.read(CsvType::ProductSubstitute)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn csv_product_substitute_dto_fixtures() -> [CsvProductSubstituteDTO; 4] {
        [
            CsvProductSubstituteDTO {
                m_product_id: "1".to_string(),
                substitute_id: "1".to_string(),
            },
            CsvProductSubstituteDTO {
                m_product_id: "1".to_string(),
                substitute_id: "2".to_string(),
            },
            CsvProductSubstituteDTO {
                m_product_id: "1".to_string(),
                substitute_id: "3".to_string(),
            },
            CsvProductSubstituteDTO {
                m_product_id: "2".to_string(),
                substitute_id: "1".to_string(),
            },
        ]
    }

    pub struct MockProductCsvDataSourceReader;

    impl CanReadCSVDataSource<CsvProductSubstituteDTO> for MockProductCsvDataSourceReader {
        fn find_all(&self) -> Result<Vec<CsvProductSubstituteDTO>, InfrastructureError> {
            Ok(csv_product_substitute_dto_fixtures().to_vec())
        }
    }
}
