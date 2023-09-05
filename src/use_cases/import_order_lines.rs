use crate::{
    domain::{
        order::Order,
        order_line::{OrderLine, OrderLineDomainFactory, OrderLinePrimaryFields},
    },
    infrastructure::{
        csv_reader::CsvOrderLineDTO,
        database::models::{order_line::OrderLineModel, OrderModel},
    },
};

use super::*;

pub struct ImportOrderLineUseCase;

impl CanReadCsvUseCase<CsvOrderLineDTO> for ImportOrderLineUseCase {}
impl CSVToEntityParser<CsvOrderLineDTO, OrderLine> for ImportOrderLineUseCase {
    fn transform_csv(&self, csv: CsvOrderLineDTO) -> Result<OrderLine, MappingError> {
        let raw_fields: Result<OrderLinePrimaryFields, MappingError> = csv.try_into();
        raw_fields.and_then(|fields| {
            let mut connection = HasTargetConnection::get_pooled_connection();
            let order_model =
                OrderModel::select_by_id(&mut connection, &fields.order_id).map_err(|e| {
                    MappingError::InfrastructureError(InfrastructureError::DatabaseError(e))
                })?;
            let order: Order = order_model.try_into()?;
            OrderLineDomainFactory::new_from_order(order, fields)
                .make()
                .map_err(|e| MappingError::DomainError(e))
        })
    }
}
impl CanPersistIntoDatabaseUseCase<OrderLine, OrderLineModel> for ImportOrderLineUseCase {
    type DbConnection = HasTargetConnection;
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
        fixtures::order_line_model_fixtures,
        infrastructure::csv_reader::CsvType,
        infrastructure::database::{
            connection::tests::HasTestConnection, models::order_line::tests::read_order_lines,
        },
        infrastructure::database::{
            connection::tests::{get_test_pooled_connection, reset_test_database},
            models::order_line::tests::insert_foreign_keys,
        },
    };

    pub struct ImportOrderLineUseCaseTest;

    impl CanReadCsvUseCase<CsvOrderLineDTO> for ImportOrderLineUseCaseTest {}
    impl CSVToEntityParser<CsvOrderLineDTO, OrderLine> for ImportOrderLineUseCaseTest {
        fn transform_csv(&self, csv: CsvOrderLineDTO) -> Result<OrderLine, MappingError> {
            let raw_fields: Result<OrderLinePrimaryFields, MappingError> = csv.try_into();
            raw_fields.and_then(|fields| {
                let mut connection = HasTestConnection::get_pooled_connection();
                let order_model = OrderModel::select_by_id(&mut connection, &fields.order_id)
                    .map_err(|e| {
                        MappingError::InfrastructureError(InfrastructureError::DatabaseError(e))
                    })?;
                let order: Order = order_model.try_into()?;
                OrderLineDomainFactory::new_from_order(order, fields)
                    .make()
                    .map_err(|e| MappingError::DomainError(e))
            })
        }
    }
    impl CanPersistIntoDatabaseUseCase<OrderLine, OrderLineModel> for ImportOrderLineUseCaseTest {
        type DbConnection = HasTestConnection;
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

        insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");

        // Result
        let use_case = ImportOrderLineUseCaseTest;
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
