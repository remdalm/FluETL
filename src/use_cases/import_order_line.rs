use std::collections::HashMap;

use log::error;

use crate::{
    domain::{
        language::Language,
        order::Order,
        order_line::{
            OrderLine, OrderLineDomainFactory, OrderLineLocalizedItemFactory,
            OrderLinePrimaryFields,
        },
        vo::localized_item::LocalizedItem,
    },
    infrastructure::{
        csv_reader::order_line::{CsvOrderLineDTO, CsvOrderLineLocalizedItemDTO},
        database::{
            batch::Config,
            connection::DbConnection,
            models::{
                order::OrderModel,
                order_line::{batch_upsert, OrderLineLangModel, OrderLineModel},
            },
        },
    },
};

use super::*;

struct ImportOrderLineItemNamesUseCase;
impl CanReadCsvUseCase<CsvOrderLineLocalizedItemDTO> for ImportOrderLineItemNamesUseCase {}
impl CanFetchLanguages for ImportOrderLineItemNamesUseCase {}
impl ImportOrderLineItemNamesUseCase {
    fn run(&self) -> Result<Vec<OrderLineLocalizedItemFactory>, UseCaseError> {
        let order_line_items_csv =
            CanReadCsvUseCase::<CsvOrderLineLocalizedItemDTO>::read(self, CsvType::OrderLineItem)?;
        let mut item_name_factories: Vec<OrderLineLocalizedItemFactory> = Vec::new();
        order_line_items_csv.into_iter().for_each(|dto| {
            let factory: Result<OrderLineLocalizedItemFactory, MappingError> = dto.try_into();
            if let Ok(factory) = factory {
                item_name_factories.push(factory);
            } else {
                error!(
                    "Failed to parse OrderLineLocalizedItemFactory: {:?}",
                    factory.unwrap_err()
                );
            }
        });

        Ok(item_name_factories)
    }

    fn make_localized_items(&self) -> Result<Vec<(u32, LocalizedItem)>, Vec<UseCaseError>> {
        debug!("Fetching languages...");
        let languages = Self::fetch_languages().map_err(|e| Vec::from([e]))?;
        debug!("Importing translations...");
        let item_name_factories = self.run().map_err(|e| Vec::from([e]))?;
        debug!("Parsing translations...");
        let item_name_results: Vec<Result<(u32, LocalizedItem), UseCaseError>> =
            item_name_factories
                .into_iter()
                .map(|factory| {
                    let language: &Language = languages
                        .iter()
                        .find(|l| l.locale() == &factory.locale)
                        .ok_or(UseCaseError::Domain(DomainError::ValidationError(format!(
                            "Language locale {} does not match item locale {}",
                            factory.locale.as_str(),
                            factory.locale.as_str()
                        ))))?;
                    let item_name = factory
                        .make_from_language(language)
                        .map_err(UseCaseError::Domain)?;
                    Ok((factory.orderline_id, item_name))
                })
                .collect();

        debug!("Parsed {} translations", item_name_results.len());
        debug!("Filtering out errors...");

        let mut item_names: Vec<(u32, LocalizedItem)> = Vec::new();
        let mut errors: Vec<UseCaseError> = Vec::new();

        item_name_results
            .into_iter()
            .for_each(|result| match result {
                Ok(item_name) => item_names.push(item_name),
                Err(error) => errors.push(error),
            });

        debug!("Filtered {} errors", errors.len());
        debug!("Total of valid translations: {}", item_names.len());

        if item_names.is_empty() {
            return Err(errors);
        }

        Ok(item_names)
    }

    fn group_localized_items(
        item_names: Vec<(u32, LocalizedItem)>,
    ) -> HashMap<u32, Vec<LocalizedItem>> {
        let mut item_names_hashmap: HashMap<u32, Vec<LocalizedItem>> = HashMap::new();
        item_names
            .into_iter()
            .for_each(|(orderline_id, item_name)| {
                if let Some(item_names_vec) = item_names_hashmap.get_mut(&orderline_id) {
                    item_names_vec.push(item_name)
                } else {
                    item_names_hashmap.insert(orderline_id, vec![item_name]);
                }
            });

        item_names_hashmap
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

impl CanReadCsvUseCase<CsvOrderLineDTO> for ImportOrderLineUseCase {}
impl CSVToEntityParser<CsvOrderLineDTO, OrderLine> for ImportOrderLineUseCase {
    fn transform_csv(&self, csv: CsvOrderLineDTO) -> Result<OrderLine, MappingError> {
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
    // fn set_batch<'a>(&'a self, models: &'a [OrderLineModel]) -> Option<Batch<OrderLineModel>> {
    //     if self.batch {
    //         Some(Batch::new(
    //             models,
    //             Some(Config::new(self.batch_size)),
    //             batch_upsert,
    //             HasTargetConnection::get_pooled_connection(),
    //         ))
    //     } else {
    //         None
    //     }
    // }
}
impl ImportCsvUseCase<CsvOrderLineDTO, OrderLine, (OrderLineModel, Vec<OrderLineLangModel>)>
    for ImportOrderLineUseCase
{
    fn get_csv_type(&self) -> CsvType {
        CsvType::OrderLine
    }
}
// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;

//     use super::*;
//     use crate::{
//         infrastructure::csv_reader::CsvType,
//         infrastructure::database::{
//             connection::tests::HasTestConnection,
//             models::{
//                 order::tests::insert_order,
//                 order_line::tests::{order_line_model_fixtures, read_order_lines},
//             },
//         },
//         infrastructure::database::{
//             connection::tests::{get_test_pooled_connection, reset_test_database},
//             models::order::bench::order_model_fixtures,
//         },
//     };

//     #[derive(Default)]
//     pub struct ImportOrderLineUseCaseTest {
//         order_cache: elsa::map::FrozenMap<u32, Box<Order>>,
//         pub use_batch: bool,
//     }

//     impl ImportOrderLineUseCaseTest {
//         fn get_order(
//             &self,
//             id: u32,
//             connection: &mut DbConnection,
//         ) -> Result<&Order, MappingError> {
//             if let Some(order) = self.order_cache.get(&id) {
//                 return Ok(order);
//             }

//             let order_model = OrderModel::select_by_id(connection, &id)
//                 .map_err(|e| MappingError::Infrastructure(InfrastructureError::DatabaseError(e)))?;
//             let order: Order = order_model.try_into()?;

//             self.order_cache.insert(id, Box::new(order));
//             let stored_order = self.order_cache.get(&id).ok_or(MappingError::Cache)?;
//             Ok(stored_order)
//         }
//     }

//     impl CanReadCsvUseCase<CsvOrderLineDTO> for ImportOrderLineUseCaseTest {}
//     impl CSVToEntityParser<CsvOrderLineDTO, OrderLine> for ImportOrderLineUseCaseTest {
//         fn transform_csv(&self, csv: CsvOrderLineDTO) -> Result<OrderLine, MappingError> {
//             let raw_fields: Result<OrderLinePrimaryFields, MappingError> = csv.try_into();
//             raw_fields.and_then(|fields| {
//                 let mut connection = HasTestConnection::get_pooled_connection();
//                 let order = self.get_order(fields.order_id, &mut connection)?.clone();
//                 OrderLineDomainFactory::new_from_order(order, &fields)
//                     .make()
//                     .map_err(MappingError::Domain)
//             })
//         }
//     }
//     impl CanPersistIntoDatabaseUseCase<OrderLine, OrderLineModel> for ImportOrderLineUseCaseTest {
//         type DbConnection = HasTestConnection;
//         fn set_batch<'a>(&'a self, models: &'a [OrderLineModel]) -> Option<Batch<OrderLineModel>> {
//             if self.use_batch {
//                 return Some(Batch::new(
//                     models,
//                     None,
//                     batch_upsert,
//                     HasTestConnection::get_pooled_connection(),
//                 ));
//             }
//             None
//         }
//     }

//     impl ImportCsvUseCase<CsvOrderLineDTO, OrderLine, OrderLineModel> for ImportOrderLineUseCaseTest {
//         fn get_csv_type(&self) -> CsvType {
//             // NamedTempFile is automatically deleted when it goes out of scope (this function ends)

//             let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//             let csv_path = root_path
//                 .join("tests")
//                 .join("fixtures")
//                 .join("order_lines_for_unit_test.csv");

//             CsvType::Test(csv_path)
//         }
//     }

//     #[test]
//     fn test_order_line_use_case() {
//         // Arrange
//         let mut connection = get_test_pooled_connection();
//         reset_test_database(&mut connection);

//         insert_order(&mut connection, false, &order_model_fixtures()[0])
//             .expect("Failed to insert Order 1");
//         insert_order(&mut connection, false, &order_model_fixtures()[1])
//             .expect("Failed to insert Order 2");

//         // Result
//         let use_case = ImportOrderLineUseCaseTest::default();
//         let errors = use_case.execute();

//         // Assert
//         assert!(
//             errors.is_none(),
//             "Failed to execute use case: {:?}",
//             errors.unwrap()
//         );
//         let persisted_order_lines = read_order_lines(&mut connection);
//         assert_eq!(persisted_order_lines.len(), 3);
//         for (i, persisted_order_line) in persisted_order_lines.iter().enumerate() {
//             assert_eq!(*persisted_order_line, order_line_model_fixtures()[i]);
//         }
//     }

//     #[test]
//     fn test_batch_order_line_use_case() {
//         // Arrange
//         let mut connection = get_test_pooled_connection();
//         reset_test_database(&mut connection);

//         insert_order(&mut connection, false, &order_model_fixtures()[0])
//             .expect("Failed to insert Order 1");
//         insert_order(&mut connection, false, &order_model_fixtures()[1])
//             .expect("Failed to insert Order 2");

//         // Result
//         let use_case = ImportOrderLineUseCaseTest {
//             use_batch: true,
//             ..Default::default()
//         };
//         let errors = use_case.execute();

//         // Assert
//         assert!(errors.is_none(), "Failed to execute use case");
//         let persisted_order_lines = read_order_lines(&mut connection);
//         assert_eq!(persisted_order_lines.len(), 3);
//         for (i, persisted_order_line) in persisted_order_lines.iter().enumerate() {
//             assert_eq!(*persisted_order_line, order_line_model_fixtures()[i]);
//         }
//     }
// }
