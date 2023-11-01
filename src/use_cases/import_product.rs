use log::debug;

use crate::{
    domain::product::{
        ProductMutateRepository, ProductReadAllRepository, ProductSubstituteReadAllRepository,
    },
    infrastructure::{
        csv_reader::product::{CsvProductSubstituteDTO, ProductCsvDataSourceReader},
        data_source::CanReadCSVDataSource,
        database::{
            batch::CanMakeBatchTransaction,
            connection::{HasConnection, HasTargetConnection},
            models::{
                product::{product_legacy_staging_model_to_lookup, ProductLegacyStagingDataSource},
                product_substitute::{ProductSubstituteModel, ProductSubstituteModelDataSource},
            },
        },
        repository::product::{
            CsvProductRepository, IdProductSubstituteLookupRepository,
            TargetDProductSubstituteRepository,
        },
    },
};

use super::{ExecutableUseCase, UseCaseError};

#[derive(Default)]
pub struct ImportProductUseCase {
    batch: bool,
    batch_size: Option<usize>, //TODO: be consistent between usize or Option<usize>
}

impl ImportProductUseCase {
    pub fn set_batch(&mut self, batch_size: usize) {
        self.batch = batch_size > 1;
        self.batch_size = Some(batch_size);
    }
}

impl ExecutableUseCase for ImportProductUseCase {
    fn execute(&self) -> Option<Vec<UseCaseError>> {
        // Product Orchestration happens here
        let substitute_lookup = IdProductSubstituteLookupRepository::new(
            ProductLegacyStagingDataSource,
            product_legacy_staging_model_to_lookup,
        )
        .find_all()
        .map_err(UseCaseError::Infrastructure);
        if let Err(e) = substitute_lookup {
            return Some(vec![e]);
        }
        let substitute_importer = ImportProductSubstitutesUseCase::new(
            CsvProductRepository::new(ProductCsvDataSourceReader),
            TargetDProductSubstituteRepository::new(
                ProductSubstituteModelDataSource,
                substitute_lookup.ok(),
                self.batch,
                self.batch_size,
                HasTargetConnection::get_pooled_connection(),
            ),
        );

        substitute_importer.execute()
    }
}

struct ImportProductSubstitutesUseCase<T, U>
where
    T: CanReadCSVDataSource<CsvProductSubstituteDTO>,
    U: CanMakeBatchTransaction<ProductSubstituteModel>,
{
    csv_repository: CsvProductRepository<T>,
    db_target_repository: TargetDProductSubstituteRepository<U>,
}

impl<T, U> ImportProductSubstitutesUseCase<T, U>
where
    T: CanReadCSVDataSource<CsvProductSubstituteDTO>,
    U: CanMakeBatchTransaction<ProductSubstituteModel>,
{
    pub fn new(
        csv_repository: CsvProductRepository<T>,
        db_target_repository: TargetDProductSubstituteRepository<U>,
    ) -> Self {
        Self {
            csv_repository,
            db_target_repository,
        }
    }
}

impl<T, U> ExecutableUseCase for ImportProductSubstitutesUseCase<T, U>
where
    T: CanReadCSVDataSource<CsvProductSubstituteDTO>,
    U: CanMakeBatchTransaction<ProductSubstituteModel>,
{
    fn execute(&self) -> Option<Vec<UseCaseError>> {
        debug!("Fetching products...");
        let (mut products, mut errors) = self.csv_repository.find_all();
        debug!("Fetched {} products", products.len());
        debug!("Fetching product substitutes...");
        let all_substitutes = self.csv_repository.find_all_substitutes().unwrap();
        debug!("Fetched {} product substitutes", all_substitutes.len());

        products.iter_mut().for_each(|product| {
            let substitutes = all_substitutes.get(product.id());
            if let Some(substitutes) = substitutes {
                errors.extend(
                    product
                        .add_substitutes(substitutes)
                        .into_iter()
                        .map(|e| e.into()),
                );
            }
        });

        if let Some(db_errors) = self
            .db_target_repository
            .save_substitutes(products.as_slice())
        {
            errors.extend(db_errors);
        }

        if errors.is_empty() {
            None
        } else {
            Some(errors.into_iter().map(UseCaseError::from).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::infrastructure::{
        csv_reader::product::tests::MockProductCsvDataSourceReader,
        data_source::CanSelectAllDataSource,
        database::{
            connection::tests::{
                get_test_pooled_connection, reset_test_database, HasTestConnection,
            },
            models::{product::ProductLegacyStagingModel, CanSelectAllModel},
        },
        repository::product::tests::ProductMockBatchTransaction,
        InfrastructureError,
    };

    struct MockProductLegacyStagingDataSource;
    impl CanSelectAllDataSource for MockProductLegacyStagingDataSource {
        type DbConnection = HasTestConnection;

        fn find_all(&self) -> Result<Vec<Self::Model>, InfrastructureError> {
            Ok(vec![
                ProductLegacyStagingModel {
                    id_source: 1,
                    id: Some(11),
                },
                ProductLegacyStagingModel {
                    id_source: 3,
                    id: Some(33),
                },
            ])
        }

        type Model = ProductLegacyStagingModel;
    }

    use super::*;

    #[test]
    #[serial]
    fn test_successful_product_substitute_import() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);
        let use_case = ImportProductSubstitutesUseCase {
            csv_repository: CsvProductRepository::new(MockProductCsvDataSourceReader),
            db_target_repository: TargetDProductSubstituteRepository::new(
                ProductMockBatchTransaction,
                IdProductSubstituteLookupRepository::new(
                    MockProductLegacyStagingDataSource,
                    product_legacy_staging_model_to_lookup,
                )
                .find_all()
                .ok(),
                false,
                None,
                HasTestConnection::get_pooled_connection(),
            ),
        };

        let errors = use_case.execute();

        assert_eq!(errors.unwrap().len(), 3); // Can't substitute itself and find product_id or substitute_id = 2 in lookup table
        assert_eq!(
            ProductSubstituteModel::select_all(&mut connection)
                .expect("Failed to select all ProductSubstituteModel"),
            [ProductSubstituteModel {
                id_product: 11,
                id_substitute: 33,
            }]
        );
    }
}
