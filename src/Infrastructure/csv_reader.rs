use crate::domain::{Order, OrderLine};
use csv::ReaderBuilder;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub enum CsvReaderError {
    IOError(std::io::Error),
    CsvParseError(csv::Error),
}

pub trait CsvReader {
    fn read_orders(&self) -> Result<Vec<Order>, CsvReaderError>;
    fn read_order_lines(&self) -> Result<Vec<OrderLine>, CsvReaderError>;
}

pub struct CsvFileReader {
    orders_file_path: String,
    order_lines_file_path: String,
}

impl CsvFileReader {
    // Private constructor, accessible only through the factory function
    fn new(orders_file_path: &str, order_lines_file_path: &str) -> Self {
        CsvFileReader {
            orders_file_path: orders_file_path.to_string(),
            order_lines_file_path: order_lines_file_path.to_string(),
        }
    }
}

impl CsvReader for CsvFileReader {
    fn read_orders(&self) -> Result<Vec<Order>, CsvReaderError> {
        let mut orders = Vec::new();

        let file = File::open(&self.orders_file_path).map_err(CsvReaderError::IOError)?;
        let mut rdr = ReaderBuilder::new().from_reader(file);

        for result in rdr.deserialize::<Order>() {
            let order = result.map_err(CsvReaderError::CsvParseError)?;
            orders.push(order);
        }

        Ok(orders)
    }

    fn read_order_lines(&self) -> Result<Vec<OrderLine>, CsvReaderError> {
        let mut order_lines = Vec::new();

        let file = File::open(&self.order_lines_file_path).map_err(CsvReaderError::IOError)?;
        let mut rdr = ReaderBuilder::new().from_reader(file);

        for result in rdr.deserialize::<OrderLine>() {
            let order_line = result.map_err(CsvReaderError::CsvParseError)?;
            order_lines.push(order_line);
        }

        Ok(order_lines)
    }
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
