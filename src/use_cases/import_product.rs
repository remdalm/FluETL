use log::debug;

use crate::{
    domain::product::{ProductMutateRepository, ProductReadAllRepository},
    infrastructure::{
        csv_reader::{
            product::{CsvProductSubstituteDTO, ProductCsvDataSourceReader},
            CanReadCSV,
        },
        database::{
            batch::CanMakeBatchTransaction,
            connection::{HasConnection, HasTargetConnection},
            models::{
                product::ProductLegacyStagingDataSourceImpl,
                product_substitute::{ProductModelDataSource, ProductSubstituteModel},
            },
        },
        repository::product::{
            CsvProductRepository, IdLookupRepository, TargetDbProductRepository,
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
    pub fn execute(&self) -> Option<Vec<UseCaseError>> {
        // Product Orchestration happens here
        let substitute_lookup = IdLookupRepository::new(ProductLegacyStagingDataSourceImpl)
            .find_all()
            .map_err(UseCaseError::Infrastructure);
        if let Err(e) = substitute_lookup {
            return Some(vec![e]);
        }
        let substitute_importer = ImportProductSubstitutesUseCase::new(
            CsvProductRepository::new(ProductCsvDataSourceReader),
            TargetDbProductRepository::new(
                ProductModelDataSource,
                substitute_lookup.ok(),
                self.batch,
                self.batch_size,
                HasTargetConnection::get_pooled_connection(),
            ),
        );

        substitute_importer.execute()
    }

    pub fn set_batch(&mut self, batch_size: usize) {
        self.batch = batch_size > 1;
        self.batch_size = Some(batch_size);
    }
}

struct ImportProductSubstitutesUseCase<T, U>
where
    T: CanReadCSV<CsvProductSubstituteDTO>,
    U: CanMakeBatchTransaction<ProductSubstituteModel>,
{
    csv_repository: CsvProductRepository<T>,
    db_target_repository: TargetDbProductRepository<U>,
}

impl<T, U> ImportProductSubstitutesUseCase<T, U>
where
    T: CanReadCSV<CsvProductSubstituteDTO>,
    U: CanMakeBatchTransaction<ProductSubstituteModel>,
{
    pub fn new(
        csv_repository: CsvProductRepository<T>,
        db_target_repository: TargetDbProductRepository<U>,
    ) -> Self {
        Self {
            csv_repository,
            db_target_repository,
        }
    }
}

impl<T, U> ExecutableUseCase for ImportProductSubstitutesUseCase<T, U>
where
    T: CanReadCSV<CsvProductSubstituteDTO>,
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
                if let Err(e) = product.add_substitutes(substitutes) {
                    errors.push(Box::new(e));
                }
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
