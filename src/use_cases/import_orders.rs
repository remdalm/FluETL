use crate::{
    domain::order::Order,
    infrastructure::{csv_reader::CsvOrderDTO, database::models::order::OrderModel},
};

use super::*;

pub struct ImportOrderUseCase;
impl CanReadCsvUseCase<CsvOrderDTO> for ImportOrderUseCase {}
impl CSVToEntityParser<CsvOrderDTO, Order> for ImportOrderUseCase {
    fn transform_csv(&self, csv: CsvOrderDTO) -> Result<Order, MappingError> {
        csv.try_into()
    }
}
impl CanPersistIntoDatabaseUseCase<Order, OrderModel> for ImportOrderUseCase {
    type DbConnection = HasTargetConnection;
}
impl ImportCsvUseCase<CsvOrderDTO, Order, OrderModel> for ImportOrderUseCase {
    fn get_csv_type(&self) -> CsvType {
        CsvType::Orders
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{
        fixtures::order_model_fixtures,
        infrastructure::database::connection::tests::{
            get_test_pooled_connection, reset_test_database, HasTestConnection,
        },
        infrastructure::database::connection::DbConnection,
        infrastructure::{csv_reader::CsvType, database::models::order::tests::read_orders},
    };

    pub struct ImportOrderUseCaseTest;
    impl CanReadCsvUseCase<CsvOrderDTO> for ImportOrderUseCaseTest {}
    impl CSVToEntityParser<CsvOrderDTO, Order> for ImportOrderUseCaseTest {
        fn transform_csv(&self, csv: CsvOrderDTO) -> Result<Order, MappingError> {
            csv.try_into()
        }
    }
    impl CanPersistIntoDatabaseUseCase<Order, OrderModel> for ImportOrderUseCaseTest {
        type DbConnection = HasTestConnection;
    }
    impl ImportCsvUseCase<CsvOrderDTO, Order, OrderModel> for ImportOrderUseCaseTest {
        fn get_csv_type(&self) -> CsvType {
            // NamedTempFile is automatically deleted when it goes out of scope (this function ends)

            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let csv_path = root_path
                .join("tests")
                .join("fixtures")
                .join("order_for_unit_test.csv");

            CsvType::Test(csv_path)
        }
    }

    #[test]
    fn test_order_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");

        // Result
        let use_case = ImportOrderUseCaseTest;
        let errors = use_case.execute();

        // Assert
        assert!(errors.is_none(), "Failed to execute use case");
        let persisted_orders = read_orders(&mut connection);
        assert_eq!(persisted_orders.len(), 2);
        assert_eq!(persisted_orders[0], order_model_fixtures()[0]);
        assert_eq!(persisted_orders[1], order_model_fixtures()[1]);
    }

    // TODO: Test with failure
}
