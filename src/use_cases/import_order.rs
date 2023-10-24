use crate::{
    domain::order::{Order, OrderDomainFactory},
    infrastructure::{
        csv_reader::{order::CsvOrderDTO, CsvType},
        database::{connection::HasTargetConnection, models::order::OrderModel},
    },
    interface_adapters::mappers::CsvEntityParser,
};

use super::{
    helpers::{
        csv::{CanReadCsvUseCase, ImportEntityCsvUseCase},
        model::CanPersistIntoDatabaseUseCase,
    },
    *,
};

pub struct ImportOrderUseCase;
impl CanReadCsvUseCase<CsvOrderDTO> for ImportOrderUseCase {}
impl CsvEntityParser<CsvOrderDTO, Order> for ImportOrderUseCase {
    fn transform_csv_row_to_entity(&self, csv: CsvOrderDTO) -> Result<Order, MappingError> {
        let factory: OrderDomainFactory = csv.try_into()?;
        factory.make().map_err(MappingError::Domain)
    }
}
impl CanPersistIntoDatabaseUseCase<Order, OrderModel> for ImportOrderUseCase {
    type DbConnection = HasTargetConnection;
}
impl ImportEntityCsvUseCase<CsvOrderDTO, Order, OrderModel> for ImportOrderUseCase {
    fn get_csv_type(&self) -> CsvType {
        CsvType::Order
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serial_test::serial;

    use super::*;
    use crate::{
        infrastructure::database::connection::tests::{
            get_test_pooled_connection, reset_test_database, HasTestConnection,
        },
        infrastructure::{
            csv_reader::CsvType,
            database::models::order::{bench::order_model_fixtures, tests::read_orders},
        },
        interface_adapters::mappers::CsvEntityParser,
        use_cases::helpers::{
            csv::{CanReadCsvUseCase, ImportEntityCsvUseCase},
            model::CanPersistIntoDatabaseUseCase,
        },
    };

    pub struct ImportOrderUseCaseTest;
    impl CanReadCsvUseCase<CsvOrderDTO> for ImportOrderUseCaseTest {}
    impl CsvEntityParser<CsvOrderDTO, Order> for ImportOrderUseCaseTest {
        fn transform_csv_row_to_entity(&self, csv: CsvOrderDTO) -> Result<Order, MappingError> {
            let factory: OrderDomainFactory = csv.try_into()?;
            factory.make().map_err(MappingError::Domain)
        }
    }
    impl CanPersistIntoDatabaseUseCase<Order, OrderModel> for ImportOrderUseCaseTest {
        type DbConnection = HasTestConnection;
    }
    impl ImportEntityCsvUseCase<CsvOrderDTO, Order, OrderModel> for ImportOrderUseCaseTest {
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
    #[serial]
    fn test_order_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // Result
        let use_case = ImportOrderUseCaseTest;
        let errors = use_case.execute();

        // Assert
        assert!(
            errors.is_none(),
            "Failed to execute use case: {:?}",
            errors.unwrap()
        );
        let persisted_orders = read_orders(&mut connection);
        assert_eq!(persisted_orders.len(), 2);
        assert_eq!(persisted_orders[0], order_model_fixtures()[0]);
        assert_eq!(persisted_orders[1], order_model_fixtures()[1]);
    }

    // TODO: Test with failure
}
