use crate::{
    domain::{
        order::Order,
        order_line::{OrderLine, OrderLineDomainFactory, OrderLinePrimaryFields},
    },
    infrastructure::{
        csv_reader::CsvOrderLineDTO,
        database::{
            connection::DbConnection,
            models::{
                order_line::{batch_upsert, OrderLineModel},
                OrderModel,
            },
        },
    },
};

use super::*;

#[derive(Default)]
pub struct ImportOrderLineUseCase {
    order_cache: elsa::map::FrozenMap<u32, Box<Order>>,
}

impl ImportOrderLineUseCase {
    fn get_order(&self, id: u32, connection: &mut DbConnection) -> Result<&Order, MappingError> {
        if let Some(order) = self.order_cache.get(&id) {
            return Ok(order);
        }

        let order_model = OrderModel::select_by_id(connection, &id).map_err(|e| {
            MappingError::InfrastructureError(InfrastructureError::DatabaseError(e))
        })?;
        let order: Order = order_model.try_into()?;

        self.order_cache.insert(id, Box::new(order));
        let stored_order = self.order_cache.get(&id).ok_or(MappingError::CacheError)?;
        Ok(stored_order)
    }
}

impl CanReadCsvUseCase<CsvOrderLineDTO> for ImportOrderLineUseCase {}
impl CSVToEntityParser<CsvOrderLineDTO, OrderLine> for ImportOrderLineUseCase {
    fn transform_csv(&self, csv: CsvOrderLineDTO) -> Result<OrderLine, MappingError> {
        let raw_fields: Result<OrderLinePrimaryFields, MappingError> = csv.try_into();
        raw_fields.and_then(|fields| {
            let mut connection = HasTargetConnection::get_pooled_connection();
            let order = self.get_order(fields.order_id, &mut connection)?.clone();
            OrderLineDomainFactory::new_from_order(order, fields)
                .make()
                .map_err(|e| MappingError::DomainError(e))
        })
    }
}
impl CanPersistIntoDatabaseUseCase<OrderLine, OrderLineModel> for ImportOrderLineUseCase {
    type DbConnection = HasTargetConnection;
    fn set_batch<'a>(&'a self, models: &'a [OrderLineModel]) -> Option<Batch<OrderLineModel>> {
        Some(Batch::new(
            models,
            batch_upsert,
            HasTargetConnection::get_pooled_connection(),
        ))
    }
}
impl ImportCsvUseCase<CsvOrderLineDTO, OrderLine, OrderLineModel> for ImportOrderLineUseCase {
    fn get_csv_type(&self) -> CsvType {
        CsvType::OrderLines
    }
}
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{
        fixtures::{order_line_model_fixtures, order_model_fixtures},
        infrastructure::csv_reader::CsvType,
        infrastructure::database::connection::tests::{
            get_test_pooled_connection, reset_test_database,
        },
        infrastructure::database::{
            connection::tests::HasTestConnection,
            models::{order::tests::insert_order, order_line::tests::read_order_lines},
        },
    };

    #[derive(Default)]
    pub struct ImportOrderLineUseCaseTest {
        order_cache: elsa::map::FrozenMap<u32, Box<Order>>,
        pub use_batch: bool,
    }

    impl ImportOrderLineUseCaseTest {
        fn get_order(
            &self,
            id: u32,
            connection: &mut DbConnection,
        ) -> Result<&Order, MappingError> {
            if let Some(order) = self.order_cache.get(&id) {
                return Ok(order);
            }

            let order_model = OrderModel::select_by_id(connection, &id).map_err(|e| {
                MappingError::InfrastructureError(InfrastructureError::DatabaseError(e))
            })?;
            let order: Order = order_model.try_into()?;

            self.order_cache.insert(id, Box::new(order));
            let stored_order = self.order_cache.get(&id).ok_or(MappingError::CacheError)?;
            Ok(stored_order)
        }
    }

    impl CanReadCsvUseCase<CsvOrderLineDTO> for ImportOrderLineUseCaseTest {}
    impl CSVToEntityParser<CsvOrderLineDTO, OrderLine> for ImportOrderLineUseCaseTest {
        fn transform_csv(&self, csv: CsvOrderLineDTO) -> Result<OrderLine, MappingError> {
            let raw_fields: Result<OrderLinePrimaryFields, MappingError> = csv.try_into();
            raw_fields.and_then(|fields| {
                let mut connection = HasTestConnection::get_pooled_connection();
                let order = self.get_order(fields.order_id, &mut connection)?.clone();
                OrderLineDomainFactory::new_from_order(order, fields)
                    .make()
                    .map_err(|e| MappingError::DomainError(e))
            })
        }
    }
    impl CanPersistIntoDatabaseUseCase<OrderLine, OrderLineModel> for ImportOrderLineUseCaseTest {
        type DbConnection = HasTestConnection;
        fn set_batch<'a>(&'a self, models: &'a [OrderLineModel]) -> Option<Batch<OrderLineModel>> {
            if self.use_batch {
                return Some(Batch::new(
                    models,
                    batch_upsert,
                    HasTestConnection::get_pooled_connection(),
                ));
            }
            None
        }
    }

    impl ImportCsvUseCase<CsvOrderLineDTO, OrderLine, OrderLineModel> for ImportOrderLineUseCaseTest {
        fn get_csv_type(&self) -> CsvType {
            // NamedTempFile is automatically deleted when it goes out of scope (this function ends)

            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let csv_path = root_path
                .join("tests")
                .join("fixtures")
                .join("order_lines_for_unit_test.csv");

            CsvType::Test(csv_path)
        }
    }

    #[test]
    fn test_order_line_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_order(&mut connection, false, &order_model_fixtures()[0])
            .expect("Failed to insert Order 1");
        insert_order(&mut connection, false, &order_model_fixtures()[1])
            .expect("Failed to insert Order 2");

        // Result
        let use_case = ImportOrderLineUseCaseTest::default();
        let errors = use_case.execute();

        // Assert
        assert!(errors.is_none(), "Failed to execute use case");
        let persisted_order_lines = read_order_lines(&mut connection);
        assert_eq!(persisted_order_lines.len(), 3);
        for (i, persisted_order_line) in persisted_order_lines.iter().enumerate() {
            assert_eq!(*persisted_order_line, order_line_model_fixtures()[i]);
        }
    }

    #[test]
    fn test_batch_order_line_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_order(&mut connection, false, &order_model_fixtures()[0])
            .expect("Failed to insert Order 1");
        insert_order(&mut connection, false, &order_model_fixtures()[1])
            .expect("Failed to insert Order 2");

        // Result
        let use_case = ImportOrderLineUseCaseTest {
            use_batch: true,
            ..Default::default()
        };
        let errors = use_case.execute();

        // Assert
        assert!(errors.is_none(), "Failed to execute use case");
        let persisted_order_lines = read_order_lines(&mut connection);
        assert_eq!(persisted_order_lines.len(), 3);
        for (i, persisted_order_line) in persisted_order_lines.iter().enumerate() {
            assert_eq!(*persisted_order_line, order_line_model_fixtures()[i]);
        }
    }
}
