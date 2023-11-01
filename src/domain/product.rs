use std::collections::HashMap;

use crate::infrastructure::InfrastructureError;

use super::{new_type::id::Id, DomainEntity, DomainError};
pub type ProductId = Id;

const SUBSTITUTE_OF_ITSELF_ERROR: &str = "A product cannot be a substitute of itself";
const IDENTICAL_SUBSTITUTE_ERROR: &str = "A product cannot have 2 identical substitutes";

// For now only product substitutes are represented but
// this structure gives the possibility to easily extend the product domain
#[derive(Debug, Clone)]
pub struct Product {
    id: ProductId,
    substitutes: Vec<ProductId>,
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Product {
    pub fn id(&self) -> &ProductId {
        &self.id
    }

    pub fn substitutes(&self) -> &[ProductId] {
        &self.substitutes
    }

    pub fn add_substitute(&mut self, substitute_id: ProductId) -> Result<(), DomainError> {
        if substitute_id == self.id {
            return Err(DomainError::ValidationError(
                SUBSTITUTE_OF_ITSELF_ERROR.to_string(),
            ));
        }
        if self.substitutes.contains(&substitute_id) {
            return Err(DomainError::ValidationError(
                IDENTICAL_SUBSTITUTE_ERROR.to_string(),
            ));
        }
        self.substitutes.push(substitute_id);
        Ok(())
    }

    pub fn add_substitutes(&mut self, substitute_ids: &[ProductId]) -> Vec<DomainError> {
        substitute_ids
            .iter()
            .filter_map(|substitute_id| self.add_substitute(*substitute_id).err())
            .collect::<Vec<DomainError>>()
    }
}

impl DomainEntity for Product {}

pub struct ProductCreationDTO {
    pub id: u32,
}

pub struct ProductDomainFactory;

impl ProductDomainFactory {
    pub fn make(dto: ProductCreationDTO) -> Result<Product, DomainError> {
        Ok(Product {
            id: ProductId::new(dto.id),
            substitutes: Vec::new(),
        })
    }
}
pub trait ProductSubstituteReadAllRepository {
    fn find_all_substitutes(
        &self,
    ) -> Result<HashMap<ProductId, Vec<ProductId>>, InfrastructureError>;
}

pub trait ProductReadAllRepository {
    fn find_all(&self) -> (Vec<Product>, Vec<Box<dyn std::error::Error>>);
}

pub trait ProductMutateRepository {
    fn save_substitutes(&self, products: &[Product]) -> Option<Vec<Box<dyn std::error::Error>>>;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn product_fixtures() -> [Product; 2] {
        [
            Product {
                id: ProductId::new(1),
                substitutes: vec![ProductId::new(2), ProductId::new(3)],
            },
            Product {
                id: ProductId::new(2),
                substitutes: vec![ProductId::new(1)],
            },
        ]
    }

    #[test]
    fn test_product_domain_factory_make() {
        let dto = ProductCreationDTO { id: 5 };
        let product = ProductDomainFactory::make(dto).unwrap();
        assert_eq!(*product.id(), ProductId::new(5));
        assert!(product.substitutes().is_empty());
    }

    #[test]
    fn test_add_valid_substitute() {
        let mut product = Product {
            id: ProductId::new(1),
            substitutes: vec![],
        };

        let substitute_id = ProductId::new(2);
        assert!(product.add_substitute(substitute_id).is_ok());
        assert_eq!(product.substitutes(), &[ProductId::new(2)]);
    }

    #[test]
    fn test_add_self_as_substitute() {
        let mut product = Product {
            id: ProductId::new(1),
            substitutes: vec![],
        };

        let substitute_id = ProductId::new(1);
        let result = product.add_substitute(substitute_id);
        assert_eq!(
            result,
            Err(DomainError::ValidationError(
                SUBSTITUTE_OF_ITSELF_ERROR.to_string()
            ))
        );
        assert!(product.substitutes().is_empty());
    }

    #[test]
    fn test_add_duplicate_substitute() {
        let mut product = Product {
            id: ProductId::new(1),
            substitutes: vec![ProductId::new(2)],
        };

        let substitute_id = ProductId::new(2);
        let result = product.add_substitute(substitute_id);
        assert_eq!(
            result,
            Err(DomainError::ValidationError(
                IDENTICAL_SUBSTITUTE_ERROR.to_string()
            ))
        );
        assert_eq!(product.substitutes(), &[ProductId::new(2)]);
    }

    #[test]
    fn test_add_multiple_valid_substitutes() {
        let mut product = Product {
            id: ProductId::new(1),
            substitutes: vec![],
        };
        let substitute_ids = vec![ProductId::new(2), ProductId::new(3)];
        let errors = product.add_substitutes(&substitute_ids);

        assert!(errors.is_empty());
        assert_eq!(product.substitutes(), &substitute_ids);
    }

    #[test]
    fn test_add_substitutes_with_errors() {
        let mut product = Product {
            id: ProductId::new(1),
            substitutes: vec![ProductId::new(2)],
        };
        let substitute_ids = vec![
            ProductId::new(2), // Duplicate
            ProductId::new(3),
            ProductId::new(1), // Self
            ProductId::new(4),
        ];
        let errors = product.add_substitutes(&substitute_ids);

        assert_eq!(errors.len(), 2);
        assert_eq!(
            errors[0],
            DomainError::ValidationError(IDENTICAL_SUBSTITUTE_ERROR.to_string())
        );
        assert_eq!(
            errors[1],
            DomainError::ValidationError(SUBSTITUTE_OF_ITSELF_ERROR.to_string())
        );
        assert_eq!(
            product.substitutes(),
            &[ProductId::new(2), ProductId::new(3), ProductId::new(4)]
        );
    }
}
