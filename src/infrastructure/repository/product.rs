use std::{cell::RefCell, collections::HashMap, error::Error};

use crate::{
    domain::product::{Product, ProductId, ProductMutateRepository, ProductReadRepository},
    infrastructure::{
        csv_reader::{product::CsvProductSubstituteDTO, CanReadCSV},
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

pub(crate) struct CsvProductRepository<T>
where
    T: CanReadCSV<CsvProductSubstituteDTO>,
{
    csv_source_reader: T,
}

impl<T> CsvProductRepository<T>
where
    T: CanReadCSV<CsvProductSubstituteDTO>,
{
    pub fn new(csv_source_reader: T) -> Self {
        Self { csv_source_reader }
    }
}

impl<T> CsvProductRepository<T> where T: CanReadCSV<CsvProductSubstituteDTO> {}

impl<T> ProductReadRepository for CsvProductRepository<T>
where
    T: CanReadCSV<CsvProductSubstituteDTO>,
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

pub(crate) struct TargetDbProductRepository<T>
where
    T: CanMakeBatchTransaction<ProductSubstituteModel>,
{
    product_substitute_data_source: T,
    use_batch: bool,
    batch_size: Option<usize>,
    connection: RefCell<DbConnection>,
}

impl<T> TargetDbProductRepository<T>
where
    T: CanMakeBatchTransaction<ProductSubstituteModel>,
{
    pub fn new(
        product_substitute_data_source: T,
        use_batch: bool,
        batch_size: Option<usize>,
        connection: DbConnection,
    ) -> Self {
        Self {
            product_substitute_data_source,
            use_batch,
            batch_size,
            connection: RefCell::new(connection),
        }
    }
}

impl<T> ProductMutateRepository for TargetDbProductRepository<T>
where
    T: CanMakeBatchTransaction<ProductSubstituteModel>,
{
    fn save_substitutes(&self, products: &[Product]) -> Option<Vec<Box<dyn std::error::Error>>> {
        let mut errors: Vec<Box<dyn Error>> = Vec::new();
        let mut models: Vec<ProductSubstituteModel> = Vec::new();
        for product in products {
            for substitute in product.substitutes() {
                models.push(ProductSubstituteModel {
                    id_product: product.id().value(),
                    id_substitute: substitute.value(),
                })
            }
        }

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

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::{
        domain::product::tests::product_fixtures,
        infrastructure::{
            csv_reader::product::tests::MockProductCsvDataSourceReader,
            database::{
                connection::{tests::HasTestConnection, HasConnection},
                models::{
                    product_substitute::tests::product_substitute_model_fixture, CanSelectAllModel,
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

    struct MockBatchTransaction;

    impl CanMakeBatchTransaction<ProductSubstituteModel> for MockBatchTransaction {
        type DbConnection = HasTestConnection;
    }

    #[test]
    #[serial]
    fn test_save_substitutes_with_batch() {
        let mock_transaction = MockBatchTransaction;
        let repo = TargetDbProductRepository::new(
            mock_transaction,
            true,
            Some(100),
            HasTestConnection::get_pooled_connection(),
        );
        let products = product_fixtures();

        let errors = repo.save_substitutes(&products);

        assert!(errors.is_none());
        assert_eq!(
            ProductSubstituteModel::select_all(&mut HasTestConnection::get_pooled_connection())
                .expect("Failed to select all ProductSubstituteModel"),
            product_substitute_model_fixture()
        )
    }

    #[test]
    #[serial]
    fn test_save_substitutes_without_batch() {
        let mock_transaction = MockBatchTransaction;
        let repo = TargetDbProductRepository::new(
            mock_transaction,
            false,
            None, // batch size
            HasTestConnection::get_pooled_connection(),
        );
        let products = product_fixtures();

        let errors = repo.save_substitutes(&products);

        assert!(errors.is_none());
        assert_eq!(
            ProductSubstituteModel::select_all(&mut HasTestConnection::get_pooled_connection())
                .expect("Failed to select all ProductSubstituteModel"),
            product_substitute_model_fixture()
        )
    }
}
