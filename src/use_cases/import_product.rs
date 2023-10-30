use log::debug;

use crate::{
    domain::product::{ProductMutateRepository, ProductReadRepository},
    infrastructure::{
        csv_reader::product::ProductCsvDataSourceReader,
        database::{
            connection::{HasConnection, HasTargetConnection},
            models::product_substitute::ProductModelDataSource,
        },
        repository::product::{CsvProductRepository, TargetDbProductRepository},
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
        let substitute_importer = ImportProductSubstitutesUseCase::new(self.batch, self.batch_size);

        substitute_importer.execute()
    }

    pub fn set_batch(&mut self, batch_size: usize) {
        self.batch = batch_size > 1;
        self.batch_size = Some(batch_size);
    }
}

struct ImportProductSubstitutesUseCase {
    csv_repository: CsvProductRepository<ProductCsvDataSourceReader>,
    db_target_repository: TargetDbProductRepository<ProductModelDataSource>,
}

impl ImportProductSubstitutesUseCase {
    pub fn new(use_batch: bool, batch_size: Option<usize>) -> Self {
        Self {
            csv_repository: CsvProductRepository::new(ProductCsvDataSourceReader),
            db_target_repository: TargetDbProductRepository::new(
                ProductModelDataSource,
                use_batch,
                batch_size,
                HasTargetConnection::get_pooled_connection(),
            ),
        }
    }
}

impl ExecutableUseCase for ImportProductSubstitutesUseCase {
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
