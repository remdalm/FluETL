use crate::infrastructure::database::batch::CanMakeBatchTransaction;
use crate::infrastructure::database::connection::{DbConnection, HasTargetConnection};
use crate::infrastructure::database::schema;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use log::debug;

use super::{CanDeleteModel, CanSelectAllModel, CanUpsertModel, Model};

#[derive(Queryable, Insertable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = schema::target::product_substitute)]
#[diesel(primary_key(id_product, id_substitute))]
pub struct ProductSubstituteModel {
    pub id_product: u32,
    pub id_substitute: u32,
}

impl Model for ProductSubstituteModel {}
impl CanUpsertModel for ProductSubstituteModel {
    fn upsert(&self, connection: &mut DbConnection) -> Result<(), DieselError> {
        super::upsert!(schema::target::product_substitute::table, self, connection)
    }
}

impl CanSelectAllModel for ProductSubstituteModel {
    fn select_all(connection: &mut DbConnection) -> Result<Vec<Self>, DieselError> {
        schema::target::product_substitute::table.load::<ProductSubstituteModel>(connection)
    }
}

impl CanDeleteModel for ProductSubstituteModel {
    fn delete_list(
        connection: &mut DbConnection,
        associations_to_delete: &[ProductSubstituteModel],
    ) -> Option<Vec<DieselError>> {
        if associations_to_delete.is_empty() {
            debug!("No associations to delete");
            return None;
        }
        use schema::target::product_substitute::dsl::{id_product, id_substitute};
        let mut errors = Vec::new();

        // Delete all associations one by one as apparently there is no bulk delete in diesel
        associations_to_delete.iter().for_each(|m| {
            let result = diesel::delete(
                schema::target::product_substitute::table
                    .filter(id_product.eq(m.id_product))
                    .filter(id_substitute.eq(m.id_substitute)),
            )
            .execute(connection)
            .map(|_| ());
            if let Err(e) = result {
                errors.push(e);
            }
        });

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }
}

pub fn product_substitute_batch_upsert(
    models: &[ProductSubstituteModel],
    connection: &mut DbConnection,
) -> Result<(), DieselError> {
    connection.transaction(|connection| {
        super::upsert!(
            schema::target::product_substitute::table,
            models,
            connection
        )
    })
}

pub struct ProductModelDataSource;

impl CanMakeBatchTransaction<ProductSubstituteModel> for ProductModelDataSource {
    type DbConnection = HasTargetConnection;
}

#[cfg(test)]
pub mod tests {
    use serial_test::serial;

    use crate::infrastructure::database::connection::tests::{
        get_test_pooled_connection, reset_test_database,
    };

    use super::*;

    pub fn product_substitute_model_fixture() -> [ProductSubstituteModel; 3] {
        [
            ProductSubstituteModel {
                id_product: 1,
                id_substitute: 2,
            },
            ProductSubstituteModel {
                id_product: 1,
                id_substitute: 3,
            },
            ProductSubstituteModel {
                id_product: 2,
                id_substitute: 1,
            },
        ]
    }

    #[test]
    #[serial]
    fn test_upsert_successful() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);
        let product_substitute = ProductSubstituteModel {
            id_product: 1,
            id_substitute: 2,
        };

        let result = product_substitute.upsert(&mut connection);

        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_select_all_successful() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        product_substitute_batch_upsert(&product_substitute_model_fixture(), &mut connection)
            .expect("Failed to insert fixtures");
        let result = ProductSubstituteModel::select_all(&mut connection);

        assert!(result.is_ok_and(|models| models == product_substitute_model_fixture().to_vec()));
    }

    #[test]
    #[serial]
    fn test_delete_list_empty() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);
        let associations_to_delete = [];

        let result = ProductSubstituteModel::delete_list(&mut connection, &associations_to_delete);

        assert!(result.is_none());
    }

    #[test]
    #[serial]
    fn test_delete_list_no_errors() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let associations_to_delete = [product_substitute_model_fixture()[1].clone()];
        product_substitute_batch_upsert(&product_substitute_model_fixture(), &mut connection)
            .expect("Failed to insert fixtures");

        let result = ProductSubstituteModel::delete_list(&mut connection, &associations_to_delete);

        assert!(result.is_none());
        assert_eq!(
            ProductSubstituteModel::select_all(&mut connection)
                .expect("Failed to select all ProductSubstituteModel"),
            [
                product_substitute_model_fixture()[0].clone(),
                product_substitute_model_fixture()[2].clone()
            ]
        );
    }

    #[test]
    #[serial]
    fn test_batch_upsert_successful() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);
        let models = product_substitute_model_fixture();

        let result = product_substitute_batch_upsert(&models, &mut connection);

        assert!(result.is_ok());

        assert_eq!(
            ProductSubstituteModel::select_all(&mut connection).unwrap(),
            product_substitute_model_fixture().to_vec()
        );
    }
}
