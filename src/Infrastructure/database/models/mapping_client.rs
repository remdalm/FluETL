use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use super::{SingleRowInsertable, SingleRowUpdatable};

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq)]
#[diesel(table_name = schema::target::mapping_client_contact)]
#[diesel(primary_key(idp_id_client))]
pub struct MappingClientModel {
    pub idp_id_client: u32,
    pub ps_id_customer: u32,
}

impl SingleRowInsertable<schema::target::mapping_client_contact::table, DbConnection>
    for MappingClientModel
{
    fn target_client_table(&self) -> schema::target::mapping_client_contact::table {
        schema::target::mapping_client_contact::table
    }
}

impl SingleRowUpdatable<schema::target::mapping_client_contact::table, DbConnection>
    for MappingClientModel
{
    fn target_client_table(&self) -> schema::target::mapping_client_contact::table {
        schema::target::mapping_client_contact::table
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

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::legacy_staging::staging_customer)]
#[diesel(primary_key(id_source_contact))]
pub struct MappingClientSource {
    pub id_source_client: i32,
    pub id_source_contact: i32,
    pub id: Option<i32>,
    // pub id_shop: u32,
    // pub m_pricelist_id: u32,
    // pub name: String,
    // pub company: Option<String>,
    // pub email: String,
    // pub active: bool,
    // pub is_xxa_centrale: bool,
    // pub free_shipping_amount: u32,
    // pub update_client: chrono::NaiveDateTime,
    // pub update_contact: chrono::NaiveDateTime,
    // pub is_synchronised: bool,
    // pub has_error: bool,
    // pub force_update: bool,
}

impl MappingClientSource {
    pub fn read(connection: &mut DbConnection) -> Result<Vec<Self>, DieselError> {
        use self::schema::legacy_staging::staging_customer::dsl::*;
        staging_customer
            .filter(id.is_not_null())
            .select(MappingClientSource::as_select())
            .load(connection)
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
pub use tests::{insert_mapping_client, update_mapping_client};

#[cfg(test)]
mod tests {
    use diesel::result::Error as DieselError;

    use crate::{
        fixtures::mapping_client_model_fixture,
        infrastructure::database::connection::tests::{
            get_test_pooled_connection, setup_test_database, teardown_test_database,
        },
    };

    use super::*;

    pub fn insert_mapping_client(connection: &mut DbConnection) -> Result<(), DieselError> {
        for mapping_client in mapping_client_model_fixture() {
            mapping_client.insert(connection)?;
        }
        Ok(())
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

        let query_result = schema::target::mapping_client_contact::dsl::mapping_client_contact
            .filter(schema::target::mapping_client_contact::idp_id_client.eq(1))
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

        let query_result = schema::target::mapping_client_contact::dsl::mapping_client_contact
            .load::<MappingClientModel>(&mut connection)
            .expect("Error loading updated MappingClientModel");

        assert_eq!(query_result.len(), mapping_client_model_fixture().len());
        assert_eq!(query_result[0].idp_id_client, 1);
        assert_eq!(query_result[0].ps_id_customer, 2);
    }
}
