use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use diesel::prelude::*;

use super::{ModelInsertOps, ModelUpdateOps};

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq)]
#[table_name = "schema::mapping_client_contact"]
#[diesel(primary_key(idp_id_client))]
pub struct MappingClientModel {
    pub idp_id_client: u32,
    pub ps_id_customer: u32,
}

impl ModelInsertOps<schema::mapping_client_contact::table, DbConnection> for MappingClientModel {
    fn target_client_table(&self) -> schema::mapping_client_contact::table {
        schema::mapping_client_contact::table
    }
}

impl ModelUpdateOps<schema::mapping_client_contact::table, DbConnection> for MappingClientModel {
    fn target_client_table(&self) -> schema::mapping_client_contact::table {
        schema::mapping_client_contact::table
    }
}

impl MappingClientModel {
    pub fn new(idp_id_client: u32, ps_id_customer: u32) -> Self {
        MappingClientModel {
            idp_id_client,
            ps_id_customer,
        }
    }
}

#[cfg(test)]
pub use tests::{insert_mapping_client, update_mapping_client};

#[cfg(test)]
mod tests {
    use diesel::result::Error as DieselError;

    use crate::tests::common::{
        get_test_pooled_connection, setup_test_database, teardown_test_database,
    };

    use super::*;

    pub fn insert_mapping_client(connection: &mut DbConnection) -> Result<(), DieselError> {
        let new_mapping_client = MappingClientModel::new(1, 1);
        new_mapping_client.insert(connection)
    }

    pub fn update_mapping_client(
        connection: &mut DbConnection,
        mapping_client_model: &MappingClientModel,
    ) -> Result<(), DieselError> {
        mapping_client_model.update(connection)
    }

    #[test]
    fn test_insert_order() {
        let mut connection = get_test_pooled_connection();

        teardown_test_database(&mut connection);
        setup_test_database(&mut connection);
        // Insert an order
        let insert_result = insert_mapping_client(&mut connection);

        assert!(insert_result.is_ok());

        let query_result = schema::mapping_client_contact::dsl::mapping_client_contact
            .filter(schema::mapping_client_contact::idp_id_client.eq(1))
            .load::<MappingClientModel>(&mut connection)
            .expect("Error loading inserted MappingClientModel");

        assert_eq!(query_result.len(), 1);
        assert_eq!(query_result[0].idp_id_client, 1);
    }

    #[test]
    fn test_update_order() {
        let mut connection = get_test_pooled_connection();

        teardown_test_database(&mut connection);
        setup_test_database(&mut connection);

        let _ = insert_mapping_client(&mut connection);
        let update_result =
            update_mapping_client(&mut connection, &mut MappingClientModel::new(1, 2));
        assert!(update_result.is_ok());

        let query_result = schema::mapping_client_contact::dsl::mapping_client_contact
            .load::<MappingClientModel>(&mut connection)
            .expect("Error loading updated MappingClientModel");

        assert_eq!(query_result.len(), 1);
        assert_eq!(query_result[0].idp_id_client, 1);
        assert_eq!(query_result[0].ps_id_customer, 2);
    }
}
