use std::error::Error;

use log::debug;

use crate::{
    domain::product::ProductReadRepository,
    infrastructure::{
        csv_reader::product::ProductCsvDataSourceReader, repository::product::CsvProductRepository,
    },
};

use super::{ExecutableUseCase, UseCaseError};

pub struct ImportProductSubstitutesUseCase {
    batch: bool,
    batch_size: usize,
    csv_repository: CsvProductRepository<ProductCsvDataSourceReader>,
}

impl ImportProductSubstitutesUseCase {
    pub fn new() -> Self {
        Self {
            batch: false,
            batch_size: 0,
            csv_repository: CsvProductRepository::new(ProductCsvDataSourceReader),
        }
    }
    pub fn set_batch(&mut self, batch_size: usize) {
        self.batch = true;
        self.batch_size = batch_size;
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

        if errors.is_empty() {
            None
        } else {
            Some(errors.into_iter().map(UseCaseError::from).collect())
        }
    }
}
