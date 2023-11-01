use crate::infrastructure::{
    database::{
        connection::{HasConnection, HasTargetConnection},
        models::{product_substitute::ProductSubstituteModel, CanDeleteModel, CanSelectAllModel},
    },
    InfrastructureError,
};

use super::{ExecutableUseCase, UseCaseError};

#[derive(Default)]
pub struct ClearProductUseCase;

impl ExecutableUseCase for ClearProductUseCase {
    fn execute(&self) -> Option<Vec<UseCaseError>> {
        ClearProductSubstitutesUseCase.execute()
    }
}

//TODO: make it testable
struct ClearProductSubstitutesUseCase;

impl ExecutableUseCase for ClearProductSubstitutesUseCase {
    fn execute(&self) -> Option<Vec<UseCaseError>> {
        let mut connection = HasTargetConnection::get_pooled_connection();
        let result = ProductSubstituteModel::select_all(&mut connection);

        if let Ok(models) = result {
            return ProductSubstituteModel::delete_list(&mut connection, &models).map(|errors| {
                errors
                    .into_iter()
                    .map(|e| UseCaseError::Infrastructure(InfrastructureError::DatabaseError(e)))
                    .collect::<Vec<UseCaseError>>()
            });
        }
        None
    }
}
