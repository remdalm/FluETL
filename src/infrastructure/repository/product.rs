use std::{cell::RefCell, collections::HashMap, error::Error};

use crate::{
    domain::product::{
        Product, ProductId, ProductMutateRepository, ProductReadAllRepository,
        ProductSubstituteReadAllRepository,
    },
    infrastructure::{
        csv_reader::product::CsvProductSubstituteDTO,
        data_source::{CanReadCSVDataSource, CanSelectAllDataSource},
        database::{
            batch::{BatchConfig, CanMakeBatchTransaction},
            connection::DbConnection,
            models::{
                product_substitute::{product_substitute_batch_upsert, ProductSubstituteModel},
                CanUpsertModel,
            },
        },
        InfrastructureError,
    },
    interface_adapters::mappers::{
        parse_string_to_u32, product::transform_csv_to_product, MappingError,
    },
};

/** CSV DATA SOURCE REPOSITORIES */
pub(crate) struct CsvProductRepository<T>
where
    T: CanReadCSVDataSource<CsvProductSubstituteDTO>,
{
    csv_source_reader: T,
}

impl<T> CsvProductRepository<T>
where
    T: CanReadCSVDataSource<CsvProductSubstituteDTO>,
{
    pub fn new(csv_source_reader: T) -> Self {
        Self { csv_source_reader }
    }
}

impl<T> CsvProductRepository<T> where T: CanReadCSVDataSource<CsvProductSubstituteDTO> {}

impl<T> ProductReadAllRepository for CsvProductRepository<T>
where
    T: CanReadCSVDataSource<CsvProductSubstituteDTO>,
{
    fn find_all(&self) -> (Vec<Product>, Vec<Box<dyn Error>>) {
        // Products has no their own source yet so we use substitutes as a source
        let mut errors: Vec<Box<dyn Error>> = Vec::new();
        let mut products: Vec<Product> = Vec::new();

        if let Err(e) = self
            .csv_source_reader
            .find_all()
            .map(|substitute_associations_csv_dto| {
                substitute_associations_csv_dto
                    .into_iter()
                    .for_each(|csv| transform_csv_to_product(csv, &mut products, &mut errors));
            })
        {
            errors.push(Box::new(e));
        }

        (products, errors)
    }
}

impl<T> ProductSubstituteReadAllRepository for CsvProductRepository<T>
where
    T: CanReadCSVDataSource<CsvProductSubstituteDTO>,
{
    fn find_all_substitutes(
        &self,
    ) -> Result<HashMap<ProductId, Vec<ProductId>>, InfrastructureError> {
        let substitute_associations_csv_dto = self.csv_source_reader.find_all()?;

        let mut substitutes: HashMap<ProductId, Vec<ProductId>> = HashMap::new();
        for csv in substitute_associations_csv_dto {
            if let Ok(dto) = TryInto::<ProductSubstituteDTO>::try_into(csv) {
                let product_id = ProductId::new(dto.product_id);
                let substitute_id = ProductId::new(dto.substitute_id);
                substitutes
                    .entry(product_id)
                    .or_default()
                    .push(substitute_id);
            }
        }

        Ok(substitutes)
    }
}

struct ProductSubstituteDTO {
    product_id: u32,
    substitute_id: u32,
}

impl TryFrom<CsvProductSubstituteDTO> for ProductSubstituteDTO {
    type Error = MappingError;

    fn try_from(dto: CsvProductSubstituteDTO) -> Result<Self, Self::Error> {
        Ok(ProductSubstituteDTO {
            product_id: parse_string_to_u32("product_id", &dto.m_product_id)?,
            substitute_id: parse_string_to_u32("substitute_id", &dto.substitute_id)?,
        })
    }
}

/***/
/** TARGET DB REPOSITORIES */
pub(crate) struct TargetDProductSubstituteRepository<T>
where
    T: CanMakeBatchTransaction<ProductSubstituteModel>,
{
    product_substitute_data_source: T,
    lookup_source: Option<HashMap<u32, u32>>,
    use_batch: bool,
    batch_size: Option<usize>,
    connection: RefCell<DbConnection>,
}

impl<T> TargetDProductSubstituteRepository<T>
where
    T: CanMakeBatchTransaction<ProductSubstituteModel>,
{
    pub fn new(
        product_substitute_data_source: T,
        lookup_source: Option<HashMap<u32, u32>>,
        use_batch: bool,
        batch_size: Option<usize>,
        connection: DbConnection,
    ) -> Self {
        Self {
            product_substitute_data_source,
            lookup_source,
            use_batch,
            batch_size,
            connection: RefCell::new(connection),
        }
    }
}

impl<T> ProductMutateRepository for TargetDProductSubstituteRepository<T>
where
    T: CanMakeBatchTransaction<ProductSubstituteModel>,
{
    fn save_substitutes(&self, products: &[Product]) -> Option<Vec<Box<dyn std::error::Error>>> {
        let mut errors: Vec<Box<dyn Error>> = Vec::new();
        let mut dirty_models: Vec<ProductSubstituteModel> = Vec::new();
        for product in products {
            for substitute in product.substitutes() {
                dirty_models.push(ProductSubstituteModel {
                    id_product: product.id().value(),
                    id_substitute: substitute.value(),
                })
            }
        }

        let models: Vec<ProductSubstituteModel> = dirty_models
            .into_iter()
            .filter_map(|m| {
                if let Some(hm) = &self.lookup_source {
                    if let (Some(id_product), Some(id_substitute)) =
                        (hm.get(&m.id_product), hm.get(&m.id_substitute))
                    {
                        Some(ProductSubstituteModel {
                            id_product: *id_product,
                            id_substitute: *id_substitute,
                        })
                    } else {
                        errors.push(Box::new(InfrastructureError::LookupError(format!(
                            "Failed to find id_product {} or id_substitute {} in lookup source",
                            m.id_product, m.id_substitute
                        ))));
                        None
                    }
                } else {
                    Some(m)
                }
            })
            .collect();

        if self.use_batch {
            let batch = self.product_substitute_data_source.make_batch(
                models.as_slice(),
                self.batch_size.map(BatchConfig::new),
                product_substitute_batch_upsert,
            );
            let batch_errors = batch.run();
            if let Some(batch_errors) = batch_errors {
                errors.extend(
                    batch_errors
                        .into_iter()
                        .map(|e| Box::new(e) as Box<dyn Error>),
                );
            }
        } else {
            let connection = &mut self.connection.borrow_mut();
            for model in models {
                if let Err(e) = model.upsert(connection) {
                    errors.push(Box::new(e));
                }
            }
        }

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }
}

/***/
/** ID LOOKUP */
pub(crate) struct IdProductSubstituteLookupRepository<T>
where
    T: CanSelectAllDataSource,
{
    data_source: T,
    transform: fn(<T as CanSelectAllDataSource>::Model, hm: &mut HashMap<u32, u32>),
}

impl<T> IdProductSubstituteLookupRepository<T>
where
    T: CanSelectAllDataSource,
{
    pub fn new(
        data_source: T,
        f: fn(<T as CanSelectAllDataSource>::Model, hm: &mut HashMap<u32, u32>),
    ) -> Self {
        Self {
            data_source,
            transform: f,
        }
    }

    pub fn find_all(&self) -> Result<HashMap<u32, u32>, InfrastructureError> {
        let mut hm: HashMap<u32, u32> = HashMap::new();
        for model in self.data_source.find_all()? {
            (self.transform)(model, &mut hm);
        }
        Ok(hm)
    }
}

#[cfg(test)]
pub mod tests {
    use serial_test::serial;

    use super::*;
    use crate::{
        domain::product::tests::product_fixtures,
        infrastructure::{
            csv_reader::product::tests::MockProductCsvDataSourceReader,
            database::{
                connection::{
                    tests::{get_test_pooled_connection, reset_test_database, HasTestConnection},
                    HasConnection,
                },
                models::{
                    product::{product_legacy_staging_model_to_lookup, ProductLegacyStagingModel},
                    product_substitute::tests::product_substitute_model_fixture,
                    CanSelectAllModel,
                },
            },
        },
    };

    #[test]
    fn test_find_all_substitutes() {
        let repo = CsvProductRepository::new(MockProductCsvDataSourceReader);
        let result = repo.find_all_substitutes().unwrap();

        assert_eq!(result.len(), 2);
        let sample_1 = result.get(&ProductId::new(1)).unwrap();
        let sample_2 = result.get(&ProductId::new(2)).unwrap();
        assert_eq!(
            sample_1,
            &vec![ProductId::new(1), ProductId::new(2), ProductId::new(3)]
        );
        assert_eq!(sample_2, &vec![ProductId::new(1)]);
    }

    #[test]
    fn test_find_all() {
        let repo = CsvProductRepository::new(MockProductCsvDataSourceReader);
        let (products, errors) = repo.find_all();

        assert_eq!(products.len(), 2);
        assert_eq!(products, product_fixtures());
        assert!(errors.is_empty());
    }

    struct MockProductLegacyStagingDataSource;

    impl CanSelectAllDataSource for MockProductLegacyStagingDataSource {
        type DbConnection = HasTestConnection;
        type Model = ProductLegacyStagingModel;

        fn find_all(&self) -> Result<Vec<Self::Model>, InfrastructureError> {
            Ok(vec![
                ProductLegacyStagingModel {
                    id_source: 1,
                    id: Some(11),
                },
                ProductLegacyStagingModel {
                    id_source: 2,
                    id: None,
                },
                ProductLegacyStagingModel {
                    id_source: 3,
                    id: Some(33),
                },
            ])
        }
    }

    #[test]
    fn test_id_lookup_repository_find_all() {
        let repo = IdProductSubstituteLookupRepository::new(
            MockProductLegacyStagingDataSource,
            product_legacy_staging_model_to_lookup,
        );

        let result = repo.find_all().unwrap();

        let mut expected: HashMap<u32, u32> = HashMap::new();
        expected.insert(1, 11);
        expected.insert(3, 33);

        assert_eq!(result, expected);
    }

    pub struct ProductMockBatchTransaction;

    impl CanMakeBatchTransaction<ProductSubstituteModel> for ProductMockBatchTransaction {
        type DbConnection = HasTestConnection;
    }

    #[test]
    #[serial]
    fn test_save_substitutes_with_batch() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);
        let mock_transaction = ProductMockBatchTransaction;
        let repo = TargetDProductSubstituteRepository::new(
            mock_transaction,
            None,
            true,
            Some(100),
            HasTestConnection::get_pooled_connection(),
        );
        let products = product_fixtures();

        let errors = repo.save_substitutes(&products);

        assert!(errors.is_none());
        assert_eq!(
            ProductSubstituteModel::select_all(&mut connection)
                .expect("Failed to select all ProductSubstituteModel"),
            product_substitute_model_fixture()
        )
    }

    #[test]
    #[serial]
    fn test_save_substitutes_without_batch() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);
        let mock_transaction = ProductMockBatchTransaction;
        let repo = TargetDProductSubstituteRepository::new(
            mock_transaction,
            None,
            false,
            None, // batch size
            HasTestConnection::get_pooled_connection(),
        );
        let products = product_fixtures();

        let errors = repo.save_substitutes(&products);

        assert!(errors.is_none());
        assert_eq!(
            ProductSubstituteModel::select_all(&mut connection)
                .expect("Failed to select all ProductSubstituteModel"),
            product_substitute_model_fixture()
        )
    }

    #[test]
    #[serial]
    fn test_save_substitutes_with_successful_lookup() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);
        let lookup_source = Some({
            let mut hm = HashMap::new();
            hm.insert(1, 11);
            hm.insert(2, 22);
            hm.insert(3, 33);
            hm
        });

        let mock_transaction = ProductMockBatchTransaction;
        let repo = TargetDProductSubstituteRepository::new(
            mock_transaction,
            lookup_source,
            false,
            None,
            HasTestConnection::get_pooled_connection(),
        );

        let products = product_fixtures();

        let errors = repo.save_substitutes(&products);

        assert!(errors.is_none());
        assert_eq!(
            ProductSubstituteModel::select_all(&mut connection)
                .expect("Failed to select all ProductSubstituteModel with lookup"),
            [
                ProductSubstituteModel {
                    id_product: 11,
                    id_substitute: 22,
                },
                ProductSubstituteModel {
                    id_product: 11,
                    id_substitute: 33,
                },
                ProductSubstituteModel {
                    id_product: 22,
                    id_substitute: 11,
                },
            ]
        )
    }

    #[test]
    #[serial]
    fn test_save_substitutes_with_failed_lookup() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);
        let lookup_source = Some({
            let mut hm = HashMap::new();
            hm.insert(1, 11);
            // Missing entry for product id 2 to induce failure
            hm.insert(3, 33);
            hm
        });

        let mock_transaction = ProductMockBatchTransaction;
        let repo = TargetDProductSubstituteRepository::new(
            mock_transaction,
            lookup_source,
            false,
            None,
            HasTestConnection::get_pooled_connection(),
        );
        let products = product_fixtures();

        let errors = repo.save_substitutes(&products);

        assert!(errors.is_some_and(|e| e.len() == 2));
        assert_eq!(
            ProductSubstituteModel::select_all(&mut connection)
                .expect("Failed to select all ProductSubstituteModel with lookup"),
            [ProductSubstituteModel {
                id_product: 11,
                id_substitute: 33,
            },]
        )
    }
}
