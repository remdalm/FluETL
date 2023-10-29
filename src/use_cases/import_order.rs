use crate::{
    domain::order::{Order, OrderDomainFactory},
    infrastructure::{
        csv_reader::{order::CsvOrderDTO, CanReadCSV, CsvType},
        database::{connection::HasTargetConnection, models::order::OrderModel},
    },
    interface_adapters::mappers::CsvEntityParser,
};

use super::{
    helpers::{csv::ImportFromSingleEntityBasedCsvUseCase, model::CanPersistIntoDatabaseUseCase},
    *,
};

pub struct ImportOrderUseCase;
impl CanReadCSV<CsvOrderDTO> for ImportOrderUseCase {
    fn find_all(&self) -> Result<Vec<CsvOrderDTO>, InfrastructureError> {
        self.read(CsvType::Order)
    }
}
impl CsvEntityParser<CsvOrderDTO, Order> for ImportOrderUseCase {
    fn transform_csv_row_to_entity(&self, csv: CsvOrderDTO) -> Result<Order, MappingError> {
        let factory: OrderDomainFactory = csv.try_into()?;
        factory.make().map_err(MappingError::Domain)
    }
}
impl CanPersistIntoDatabaseUseCase<Order, OrderModel> for ImportOrderUseCase {
    type DbConnection = HasTargetConnection;
}
impl ImportFromSingleEntityBasedCsvUseCase<CsvOrderDTO, Order, OrderModel> for ImportOrderUseCase {}

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
            csv::ImportFromSingleEntityBasedCsvUseCase, model::CanPersistIntoDatabaseUseCase,
        },
    };

    pub struct ImportOrderUseCaseTest;
    impl CanReadCSV<CsvOrderDTO> for ImportOrderUseCaseTest {
        fn find_all(&self) -> Result<Vec<CsvOrderDTO>, InfrastructureError> {
            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let csv_path = root_path
                .join("tests")
                .join("fixtures")
                .join("order_for_unit_test.csv");
            self.read(CsvType::Test(csv_path))
        }
    }
    impl CsvEntityParser<CsvOrderDTO, Order> for ImportOrderUseCaseTest {
        fn transform_csv_row_to_entity(&self, csv: CsvOrderDTO) -> Result<Order, MappingError> {
            let factory: OrderDomainFactory = csv.try_into()?;
            factory.make().map_err(MappingError::Domain)
        }
    }
    impl CanPersistIntoDatabaseUseCase<Order, OrderModel> for ImportOrderUseCaseTest {
        type DbConnection = HasTestConnection;
    }
    impl ImportFromSingleEntityBasedCsvUseCase<CsvOrderDTO, Order, OrderModel>
        for ImportOrderUseCaseTest
    {
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
