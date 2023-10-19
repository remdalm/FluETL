use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CsvOrderDTO {
    pub c_order_id: String,
    pub c_bpartner_id: String,
    pub client_name: String,
    pub date: String,
    pub order_ref: String,
    pub po_ref: String,
    pub origin: String,
    pub completion: String,
    pub order_status: String,
    pub delivery_status: String,
}

impl CsvDTO for CsvOrderDTO {}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    pub const DELIMITER: u8 = b';';

    pub const ORDER_FLAWLESS_CSV: &str= "c_order_id;c_bpartner_id;client_name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Client 1;2023-08-01;Ref1;PoRef1;Web;30;CO;CO\n2;2;Client 2;2023-08-02;Ref2;PoRef2;EDI;20;IN;CO\n";
    pub const ORDER_WITH_EMPTY_FIELD_CSV: &str = "c_order_id;c_bpartner_id;client_name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Client 1;2023-08-01;Ref1;PoRef1;Web;30;;\n3;1;;2023-08-03;Ref3;PoRef3;Origin3;0;;CO\n";
    pub const ORDER_WITH_MISSING_DATA_CSV: &str = "c_order_id;c_bpartner_id;client_name;date;order_ref;po_ref;origin;completion;order_status;delivery_status\n1;1;Client 1;2023-08-01;Ref1;PoRef1;Web;30\n2;2;Client 2;2023-08-02;Ref2;PoRef2;EDI;20\n";

    pub fn csv_order_dto_fixtures() -> [CsvOrderDTO; 3] {
        [
            CsvOrderDTO {
                c_order_id: 1.to_string(),
                c_bpartner_id: 1.to_string(),
                client_name: "Client 1".to_string(),
                date: "2023-08-01".to_string(),
                order_ref: "Ref1".to_string(),
                po_ref: "PoRef1".to_string(),
                origin: "Web".to_string(),
                completion: "30".to_string(),
                order_status: "CO".to_string(),
                delivery_status: "CO".to_string(),
            },
            CsvOrderDTO {
                c_order_id: 2.to_string(),
                c_bpartner_id: 2.to_string(),
                client_name: "Client 2".to_string(),
                date: "2023-08-02".to_string(),
                order_ref: "Ref2".to_string(),
                po_ref: "PoRef2".to_string(),
                origin: "EDI".to_string(),
                completion: "20".to_string(),
                order_status: "IN".to_string(),
                delivery_status: "CO".to_string(),
            },
            CsvOrderDTO {
                c_order_id: 3.to_string(),
                c_bpartner_id: 1.to_string(),
                client_name: String::new(),
                date: "2023-08-03".to_string(),
                order_ref: "Ref3".to_string(),
                po_ref: "PoRef3".to_string(),
                origin: "Origin3".to_string(),
                completion: "0".to_string(),
                order_status: String::new(),
                delivery_status: "CO".to_string(),
            },
        ]
    }

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
            panic!("Expected InfrastructureError::FileNotFound");
        }
    }
}
