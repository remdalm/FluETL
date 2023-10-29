use crate::{
    domain::product::{Product, ProductCreationDTO, ProductDomainFactory},
    infrastructure::csv_reader::product::CsvProductSubstituteDTO,
};

use super::{parse_string_to_u32, MappingError};

pub fn transform_csv_to_product(
    csv: CsvProductSubstituteDTO,
    products: &mut Vec<Product>,
    errors: &mut Vec<Box<dyn std::error::Error>>,
) {
    let result: Result<ProductCreationDTO, MappingError> = csv.try_into();
    match result {
        Ok(dto) => match ProductDomainFactory::make(dto) {
            Ok(product) => {
                if !products.contains(&product) {
                    products.push(product);
                }
            }
            Err(e) => errors.push(Box::new(e)),
        },
        Err(e) => errors.push(Box::new(e)),
    }
}

impl TryFrom<CsvProductSubstituteDTO> for ProductCreationDTO {
    type Error = MappingError;

    fn try_from(csv: CsvProductSubstituteDTO) -> Result<Self, Self::Error> {
        Ok(ProductCreationDTO {
            id: parse_string_to_u32("product_id", &csv.m_product_id)?,
        })
    }
}
