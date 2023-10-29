use std::{collections::HashMap, error::Error};

use crate::{
    domain::product::{Product, ProductId, ProductReadRepository},
    infrastructure::{
        csv_reader::{product::CsvProductSubstituteDTO, CanReadCSV},
        InfrastructureError,
    },
    interface_adapters::mappers::{
        parse_string_to_u32, product::transform_csv_to_product, MappingError,
    },
};

pub struct CsvProductRepository<T>
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

        match self.csv_source_reader.find_all() {
            Ok(substitute_associations_csv_dto) => {
                substitute_associations_csv_dto
                    .into_iter()
                    .for_each(|csv| transform_csv_to_product(csv, &mut products, &mut errors));
            }
            Err(e) => {
                errors.push(Box::new(e));
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::product::tests::product_fixtures,
        infrastructure::csv_reader::product::tests::MockProductCsvDataSourceReader,
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
}
