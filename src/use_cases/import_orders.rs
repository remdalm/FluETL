use crate::{
    domain::Order,
    infrastructure::{csv_reader::CsvOrderDTO, database::models::order::OrderModel},
};

use super::*;

pub struct ImportOrderUseCase;

pub struct OrderManager;

impl UseCaseImportManager<CsvOrderDTO, Order, OrderModel> for OrderManager {}
impl CanReadCsvUseCase<CsvOrderDTO, Order> for OrderManager {}
impl CanPersistIntoDatabaseUseCase<Order, OrderModel> for OrderManager {}
impl ImportCsvUseCase<CsvOrderDTO, Order, OrderModel> for ImportOrderUseCase {
    type ManagerImpl = OrderManager;

    fn get_csv_type(&self) -> CsvType {
        CsvType::Orders
    }

    fn concrete_manager(&self) -> Self::ManagerImpl {
        OrderManager
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{
        benches::database_connection::tests::{get_test_pooled_connection, reset_test_database},
        fixtures::order_model_fixtures,
        infrastructure::{
            csv_reader::CsvType,
            database::models::order::tests::{insert_foreign_keys, read_orders},
        },
    };

    pub struct ImportOrderUseCaseTest;
    pub struct OrderManagerTest;

    impl UseCaseImportManager<CsvOrderDTO, Order, OrderModel> for OrderManagerTest {}
    impl CanReadCsvUseCase<CsvOrderDTO, Order> for OrderManagerTest {}
    impl CanPersistIntoDatabaseUseCase<Order, OrderModel> for OrderManagerTest {
        fn get_pooled_connection(&self) -> DbConnection {
            get_test_pooled_connection()
        }
    }

    impl ImportCsvUseCase<CsvOrderDTO, Order, OrderModel> for ImportOrderUseCaseTest {
        type ManagerImpl = OrderManagerTest;

        fn get_csv_type(&self) -> CsvType {
            // NamedTempFile is automatically deleted when it goes out of scope (this function ends)

            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let csv_path = root_path
                .join("tests")
                .join("fixtures")
                .join("order_for_unit_test.csv");

            CsvType::Test(csv_path)
        }

        fn concrete_manager(&self) -> Self::ManagerImpl {
            OrderManagerTest
        }
    }

    #[test]
    fn test_order_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");

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
