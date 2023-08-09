use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

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

#[derive(Debug)]
pub enum CsvReaderError {
    IOError(std::io::Error),
    CsvParseError(csv::Error),
}

pub trait CsvReader {
    fn read_orders(&self) -> Result<Vec<CsvOrderDTO>, CsvReaderError>;
    // fn read_order_lines(&self) -> Result<Vec<OrderLine>, CsvReaderError>;
}

#[derive(Debug)]
pub struct CsvFileReader {
    orders_file_path: Option<String>,
    order_lines_file_path: Option<String>,
}

impl CsvFileReader {
    // Private constructor, accessible only through the factory function
    fn new(orders_file_path: &str, order_lines_file_path: &str) -> Self {
        CsvFileReader {
            orders_file_path: Some(orders_file_path.to_string()),
            order_lines_file_path: Some(order_lines_file_path.to_string()),
        }
    }

    fn only_orders(orders_file_path: &str) -> Self {
        CsvFileReader {
            orders_file_path: Some(orders_file_path.to_string()),
            order_lines_file_path: None,
        }
    }
}

impl CsvReader for CsvFileReader {
    fn read_orders(&self) -> Result<Vec<CsvOrderDTO>, CsvReaderError> {
        let mut csv_orders = Vec::new();

        let file =
            File::open(self.orders_file_path.as_ref().unwrap()).map_err(CsvReaderError::IOError)?;
        let mut rdr = ReaderBuilder::new().from_reader(file);

        for result in rdr.deserialize::<CsvOrderDTO>() {
            let csv_order = result.map_err(CsvReaderError::CsvParseError)?;
            csv_orders.push(csv_order);
        }

        Ok(csv_orders)
    }

    // fn read_order_lines(&self) -> Result<Vec<OrderLine>, CsvReaderError> {
    //     let mut order_lines = Vec::new();

    //     let file = File::open(&self.order_lines_file_path).map_err(CsvReaderError::IOError)?;
    //     let mut rdr = ReaderBuilder::new().from_reader(file);

    //     // for result in rdr.deserialize::<OrderLine>() {
    //     //     let order_line = result.map_err(CsvReaderError::CsvParseError)?;
    //     //     order_lines.push(order_line);
    //     // }

    //     Ok(order_lines)
    // }
}

// Factory function to create instances of CsvFileReader and validate file paths
pub fn create_csv_file_reader(
    orders_file_path: &str,
    order_lines_file_path: &str,
) -> Result<CsvFileReader, String> {
    let orders_file_path = super::environment::get_env("ORDERS_FILE_PATH")?;
    let order_lines_file_path = super::environment::get_env("ORDER_LINES_FILE_PATH")?;

    // Perform validation on the file paths here...
    if !Path::new(&orders_file_path).exists() {
        return Err(format!("Orders file not found: {}", orders_file_path));
    }

    if !Path::new(&order_lines_file_path).exists() {
        return Err(format!(
            "Order lines file not found: {}",
            order_lines_file_path
        ));
    }

    Ok(CsvFileReader::new(
        &orders_file_path,
        &order_lines_file_path,
    ))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::fixtures::csv_order_dto_fixtures;
    use crate::tests::fixtures::ORDER_CSV;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Helper function to create a temporary CSV file for testing
    fn create_temp_csv(content: &str) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp CSV file");
        temp_file
            .write_all(content.as_bytes())
            .expect("Failed to write to temp CSV file");
        temp_file
    }

    #[test]
    fn test_read_orders_success() {
        let temp_csv = create_temp_csv(ORDER_CSV);
        let csv_file_reader = CsvFileReader::only_orders(temp_csv.path().to_str().unwrap());

        let order_fixture = csv_order_dto_fixtures();

        // Act
        let result = csv_file_reader.read_orders();

        //Assert
        assert!(result.is_ok(), "Expected successful read_orders");
        let orders = result.unwrap();
        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0], order_fixture[0]);
        assert_eq!(orders[1], order_fixture[1]);
    }

    // #[test]
    // fn test_read_order_lines_success() {
    //     // Arrange
    //     let order_lines_csv = "c_orderline_id,c_order_id,product_ref\n1,1,Product 1\n";
    //     let temp_csv = create_temp_csv(order_lines_csv);
    //     let csv_file_reader = CsvFileReader::new("", temp_csv.path().to_str().unwrap());

    //     // Act
    //     let result = csv_file_reader.read_order_lines();

    //     // Assert
    //     assert!(result.is_ok(), "Expected successful read_order_lines");
    //     let order_lines = result.unwrap();
    //     assert_eq!(order_lines.len(), 1);
    //     assert_eq!(order_lines[0].c_orderline_id, 1);
    //     assert_eq!(order_lines[0].c_order_id, 1);
    //     assert_eq!(order_lines[0].product_ref, "Product 1");
    //     assert_eq!(order_lines[0].product_name, "");
    //     assert_eq!(order_lines[0].qty_ordered, 0);
    //     assert_eq!(order_lines[0].qty_reserved, 0);
    //     assert_eq!(order_lines[0].qty_delivered, 0);
    //     assert_eq!(
    //         order_lines[0].due_date,
    //         chrono::NaiveDate::from_ymd_opt(2023, 8, 1).expect("Invalid date")
    //     );
    // }

    // #[test]
    // fn test_create_csv_file_reader_success() {
    //     // Arrange
    //     let orders_csv =
    //         "c_order_id,c_bpartner_id,name,date,order_ref\n1,1,Order 1,2023-08-01,Ref1\n";
    //     let order_lines_csv = "c_orderline_id,c_order_id,product_ref\n1,1,Product 1\n";
    //     let temp_orders_csv = create_temp_csv(orders_csv);
    //     let temp_order_lines_csv = create_temp_csv(order_lines_csv);

    //     // Act
    //     let result = create_csv_file_reader(
    //         temp_orders_csv.path().to_str().unwrap(),
    //         temp_order_lines_csv.path().to_str().unwrap(),
    //     );

    //     // Assert
    //     assert!(result.is_ok(), "Expected successful create_csv_file_reader");
    //     let csv_file_reader = result.unwrap();
    //     assert_eq!(
    //         csv_file_reader.orders_file_path,
    //         temp_orders_csv.path().to_str().unwrap()
    //     );
    //     assert_eq!(
    //         csv_file_reader.order_lines_file_path,
    //         temp_order_lines_csv.path().to_str().unwrap()
    //     );
    // }

    // #[test]
    // fn test_create_csv_file_reader_invalid_paths() {
    //     // Arrange: Use non-existent paths
    //     let orders_file_path = "/nonexistent/orders.csv";
    //     let order_lines_file_path = "/nonexistent/order_lines.csv";

    //     // Act
    //     let result = create_csv_file_reader(orders_file_path, order_lines_file_path);

    //     // Assert
    //     assert!(result.is_err(), "Expected error for invalid paths");
    //     let error_message = result.unwrap_err();
    //     assert!(
    //         error_message.contains("Orders file not found"),
    //         "Error message should mention Orders file not found"
    //     );
    //     assert!(
    //         error_message.contains("Order lines file not found"),
    //         "Error message should mention Order lines file not found"
    //     );
    // }
}
