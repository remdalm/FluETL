use std::collections::HashMap;

use crate::{
    domain::{
        order::Order,
        order_line::{
            OrderLine, OrderLineDomainFactory, OrderLineLocalizedItemFactory,
            OrderLinePrimaryFields,
        },
        vo::localized_item::LocalizedItem,
    },
    infrastructure::{
        csv_reader::{
            order_line::{CsvOrderLineDTO, CsvOrderLineLocalizedItemDTO},
            CsvType,
        },
        data_source::CanReadCSVDataSource,
        database::{
            batch::{Batch, BatchConfig},
            connection::{DbConnection, HasConnection, HasTargetConnection},
            models::{
                order::OrderModel,
                order_line::{batch_upsert, OrderLineLangModel, OrderLineModel},
            },
        },
        InfrastructureError,
    },
    interface_adapters::mappers::CsvEntityParser,
};

use super::{
    helpers::{
        csv::ImportFromSingleEntityBasedCsvUseCase, language::CanFetchLanguages,
        localized_item::ImportLocalizedItem, model::CanPersistIntoDatabaseUseCase,
    },
    *,
};

struct ImportOrderLineItemNamesUseCase;
impl CanReadCSVDataSource<CsvOrderLineLocalizedItemDTO> for ImportOrderLineItemNamesUseCase {
    fn find_all(&self) -> Result<Vec<CsvOrderLineLocalizedItemDTO>, InfrastructureError> {
        self.read(CsvType::OrderLineItem)
    }
}
impl CanFetchLanguages for ImportOrderLineItemNamesUseCase {}
impl ImportLocalizedItem<OrderLineLocalizedItemFactory, CsvOrderLineLocalizedItemDTO>
    for ImportOrderLineItemNamesUseCase
{
    fn source(&self) -> Result<Vec<CsvOrderLineLocalizedItemDTO>, UseCaseError> {
        self.find_all().map_err(|e| e.into())
    }
}

#[derive(Default)]
pub struct ImportOrderLineUseCase {
    order_cache: elsa::map::FrozenMap<u32, Box<Order>>,
    item_names: HashMap<u32, Vec<LocalizedItem>>,
    batch: bool,
    batch_size: usize,
}

impl ImportOrderLineUseCase {
    pub fn new() -> Result<Self, Vec<UseCaseError>> {
        let item_names = ImportOrderLineItemNamesUseCase.make_localized_items()?;

        Ok(Self {
            item_names: ImportOrderLineItemNamesUseCase::group_localized_items(item_names),
            ..Self::default()
        })
    }

    pub fn set_batch(&mut self, batch_size: usize) {
        self.batch = true;
        self.batch_size = batch_size;
    }

    fn get_order(&self, id: u32, connection: &mut DbConnection) -> Result<&Order, MappingError> {
        if let Some(order) = self.order_cache.get(&id) {
            return Ok(order);
        }

        let order_model = OrderModel::select_by_id(connection, &id)
            .map_err(|e| MappingError::Infrastructure(InfrastructureError::DatabaseError(e)))?;
        let order: Order = order_model.try_into()?;

        self.order_cache.insert(id, Box::new(order));
        let stored_order = self.order_cache.get(&id).ok_or(MappingError::Cache)?;
        Ok(stored_order)
    }
}

impl CanReadCSVDataSource<CsvOrderLineDTO> for ImportOrderLineUseCase {
    fn find_all(&self) -> Result<Vec<CsvOrderLineDTO>, InfrastructureError> {
        self.read(CsvType::OrderLine)
    }
}
impl CsvEntityParser<CsvOrderLineDTO, OrderLine> for ImportOrderLineUseCase {
    fn transform_csv_row_to_entity(&self, csv: CsvOrderLineDTO) -> Result<OrderLine, MappingError> {
        let raw_fields: Result<OrderLinePrimaryFields, MappingError> = csv.try_into();
        raw_fields.and_then(|fields| {
            let mut connection = HasTargetConnection::get_pooled_connection();
            let order = self.get_order(fields.order_id, &mut connection)?.clone();
            let mut factory = OrderLineDomainFactory::new_from_order(order, &fields);

            self.item_names.contains_key(&fields.orderline_id).then(|| {
                factory.item_names = self
                    .item_names
                    .get(&fields.orderline_id)
                    .unwrap()
                    .to_owned();
            });
            factory.make().map_err(MappingError::Domain)
        })
    }
}
impl CanPersistIntoDatabaseUseCase<OrderLine, (OrderLineModel, Vec<OrderLineLangModel>)>
    for ImportOrderLineUseCase
{
    type DbConnection = HasTargetConnection;
    fn set_batch<'a>(
        &'a self,
        models: &'a [(OrderLineModel, Vec<OrderLineLangModel>)],
    ) -> Option<Batch<(OrderLineModel, Vec<OrderLineLangModel>)>> {
        if self.batch {
            Some(Batch::new(
                models,
                Some(BatchConfig::new(self.batch_size)),
                batch_upsert,
                HasTargetConnection::get_pooled_connection(),
            ))
        } else {
            None
        }
    }
}
impl
    ImportFromSingleEntityBasedCsvUseCase<
        CsvOrderLineDTO,
        OrderLine,
        (OrderLineModel, Vec<OrderLineLangModel>),
    > for ImportOrderLineUseCase
{
}
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serial_test::serial;

    use super::*;
    use crate::{
        domain::vo::localized_item::tests::localized_item_fixtures,
        infrastructure::csv_reader::CsvType,
        infrastructure::database::{
            connection::tests::HasTestConnection,
            models::{
                order::tests::insert_order,
                order_line::tests::{
                    order_line_lang_model_fixtures, order_line_model_fixtures,
                    read_order_line_items, read_order_lines,
                },
            },
        },
        infrastructure::database::{
            connection::tests::{get_test_pooled_connection, reset_test_database},
            models::order::bench::order_model_fixtures,
        },
    };

    struct ImportOrderLineItemNamesUseCaseTest;
    impl CanReadCSVDataSource<CsvOrderLineLocalizedItemDTO> for ImportOrderLineItemNamesUseCaseTest {
        fn find_all(&self) -> Result<Vec<CsvOrderLineLocalizedItemDTO>, InfrastructureError> {
            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let csv_path = root_path
                .join("tests")
                .join("fixtures")
                .join("order_lines_items_for_unit_test.csv");
            self.read(CsvType::Test(csv_path))
        }
    }
    impl CanFetchLanguages for ImportOrderLineItemNamesUseCaseTest {}
    impl ImportLocalizedItem<OrderLineLocalizedItemFactory, CsvOrderLineLocalizedItemDTO>
        for ImportOrderLineItemNamesUseCaseTest
    {
        // Mock method
        fn make_localized_items(&self) -> Result<Vec<(u32, LocalizedItem)>, Vec<UseCaseError>> {
            Ok(vec![
                (1, localized_item_fixtures()[0].clone()),
                (1, localized_item_fixtures()[1].clone()),
                (2, localized_item_fixtures()[2].clone()),
            ])
        }
        fn source(&self) -> Result<Vec<CsvOrderLineLocalizedItemDTO>, UseCaseError> {
            self.find_all().map_err(|e| e.into())
        }
    }

    #[derive(Default)]
    pub struct ImportOrderLineUseCaseTest {
        order_cache: elsa::map::FrozenMap<u32, Box<Order>>,
        item_names: HashMap<u32, Vec<LocalizedItem>>,
        pub use_batch: bool,
    }

    impl ImportOrderLineUseCaseTest {
        pub fn new() -> Result<Self, Vec<UseCaseError>> {
            let item_names = ImportOrderLineItemNamesUseCaseTest.make_localized_items()?;

            Ok(Self {
                item_names: ImportOrderLineItemNamesUseCaseTest::group_localized_items(item_names),
                ..Self::default()
            })
        }
        fn get_order(
            &self,
            id: u32,
            connection: &mut DbConnection,
        ) -> Result<&Order, MappingError> {
            if let Some(order) = self.order_cache.get(&id) {
                return Ok(order);
            }

            let order_model = OrderModel::select_by_id(connection, &id)
                .map_err(|e| MappingError::Infrastructure(InfrastructureError::DatabaseError(e)))?;
            let order: Order = order_model.try_into()?;

            self.order_cache.insert(id, Box::new(order));
            let stored_order = self.order_cache.get(&id).ok_or(MappingError::Cache)?;
            Ok(stored_order)
        }
    }

    impl CanReadCSVDataSource<CsvOrderLineDTO> for ImportOrderLineUseCaseTest {
        fn find_all(&self) -> Result<Vec<CsvOrderLineDTO>, InfrastructureError> {
            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let csv_path = root_path
                .join("tests")
                .join("fixtures")
                .join("order_lines_for_unit_test.csv");
            self.read(CsvType::Test(csv_path))
        }
    }
    impl CsvEntityParser<CsvOrderLineDTO, OrderLine> for ImportOrderLineUseCaseTest {
        fn transform_csv_row_to_entity(
            &self,
            csv: CsvOrderLineDTO,
        ) -> Result<OrderLine, MappingError> {
            let raw_fields: Result<OrderLinePrimaryFields, MappingError> = csv.try_into();
            // todo!()
            raw_fields.and_then(|fields| {
                let mut connection = HasTestConnection::get_pooled_connection();
                let order = self.get_order(fields.order_id, &mut connection)?.clone();
                let mut factory = OrderLineDomainFactory::new_from_order(order, &fields);

                self.item_names.contains_key(&fields.orderline_id).then(|| {
                    factory.item_names = self
                        .item_names
                        .get(&fields.orderline_id)
                        .unwrap()
                        .to_owned();
                });
                factory.make().map_err(MappingError::Domain)
            })
        }
    }
    impl CanPersistIntoDatabaseUseCase<OrderLine, (OrderLineModel, Vec<OrderLineLangModel>)>
        for ImportOrderLineUseCaseTest
    {
        type DbConnection = HasTestConnection;
        fn set_batch<'a>(
            &'a self,
            models: &'a [(OrderLineModel, Vec<OrderLineLangModel>)],
        ) -> Option<Batch<(OrderLineModel, Vec<OrderLineLangModel>)>> {
            if self.use_batch {
                return Some(Batch::new(
                    models,
                    None,
                    batch_upsert,
                    HasTestConnection::get_pooled_connection(),
                ));
            }
            None
        }
    }

    impl
        ImportFromSingleEntityBasedCsvUseCase<
            CsvOrderLineDTO,
            OrderLine,
            (OrderLineModel, Vec<OrderLineLangModel>),
        > for ImportOrderLineUseCaseTest
    {
    }

    #[test]
    #[serial]
    fn test_order_line_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_order(&mut connection, false, &order_model_fixtures()[0])
            .expect("Failed to insert Order 1");
        insert_order(&mut connection, false, &order_model_fixtures()[1])
            .expect("Failed to insert Order 2");

        // Result
        let use_case = ImportOrderLineUseCaseTest::new().unwrap();
        let errors = use_case.execute();

        // Assert
        assert!(
            errors.is_none(),
            "Failed to execute use case: {:?}",
            errors.unwrap()
        );
        let persisted_order_lines = read_order_lines(&mut connection);
        assert_eq!(persisted_order_lines.len(), 3);

        for (i, persisted_order_line) in persisted_order_lines.iter().enumerate() {
            assert_eq!(*persisted_order_line, order_line_model_fixtures()[i]);
            let order_line_items =
                read_order_line_items(&mut connection, &order_line_model_fixtures()[i]);
            assert_eq!(order_line_items, order_line_lang_model_fixtures()[i]);
        }
    }

    #[test]
    #[serial]
    fn test_batch_order_line_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_order(&mut connection, false, &order_model_fixtures()[0])
            .expect("Failed to insert Order 1");
        insert_order(&mut connection, false, &order_model_fixtures()[1])
            .expect("Failed to insert Order 2");

        // Result
        let mut use_case = ImportOrderLineUseCaseTest::new().unwrap();
        use_case.use_batch = true;

        let errors = use_case.execute();

        // Assert
        assert!(errors.is_none(), "Failed to execute use case");
        let persisted_order_lines = read_order_lines(&mut connection);
        assert_eq!(persisted_order_lines.len(), 3);

        for (i, persisted_order_line) in persisted_order_lines.iter().enumerate() {
            assert_eq!(*persisted_order_line, order_line_model_fixtures()[i]);
            let order_line_items =
                read_order_line_items(&mut connection, &order_line_model_fixtures()[i]);
            assert_eq!(order_line_items, order_line_lang_model_fixtures()[i]);
        }
    }
}
