use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use super::{CanSelectAllModel, CanUpsertModel, Model, SingleRowInsertable, SingleRowUpdatable};

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq)]
#[diesel(table_name = schema::target::mapping_client_contact)]
#[diesel(primary_key(id_customer))]
pub struct MappingClientModel {
    pub id_customer: u32,
    pub idp_id_client: u32,
}

impl Model for MappingClientModel {}
impl CanUpsertModel for MappingClientModel {
    fn upsert(&self, connection: &mut DbConnection) -> Result<(), DieselError> {
        diesel::insert_into(schema::target::mapping_client_contact::table)
            .values(self)
            .on_conflict(diesel::dsl::DuplicatedKeys)
            .do_update()
            .set(self)
            .execute(connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }
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
    pub fn new(id_customer: u32, idp_id_client: u32) -> Self {
        MappingClientModel {
            id_customer,
            idp_id_client,
        }
    }
}

#[derive(Queryable, Identifiable, Selectable, Default, Clone, Debug)]
#[diesel(table_name = schema::legacy_staging::staging_customer)]
#[diesel(primary_key(id_source_contact))]
pub struct MappingClientSource {
    pub id_source_client: i32,
    pub id_source_contact: i32,
    pub id: Option<i32>,
    // pub id_shop: u32,
    // pub m_pricelist_id: u32,
    // pub client_name: String,
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

impl Model for MappingClientSource {}
impl CanSelectAllModel for MappingClientSource {
    fn select_all(connection: &mut DbConnection) -> Result<Vec<Self>, DieselError> {
        use self::schema::legacy_staging::staging_customer::dsl::*;
        staging_customer
            .filter(id.is_not_null())
            .select(MappingClientSource::as_select())
            .load(connection)
            .map_err(|e| e.into())
    }
}

// Use for benchmarking
pub mod bench {
    use super::*;
    pub fn mapping_client_model_fixture() -> [MappingClientModel; 2] {
        [MappingClientModel::new(1, 1), MappingClientModel::new(2, 2)]
    }
}

#[cfg(test)]
pub mod tests {
    use diesel::result::Error as DieselError;

    use crate::infrastructure::database::connection::tests::{
        get_test_pooled_connection, reset_test_database,
    };

    pub fn mapping_client_source_model_fixture() -> [MappingClientSource; 2] {
        [
            MappingClientSource {
                id_source_client: 1,
                id_source_contact: 1,
                id: Some(1),
            },
            MappingClientSource {
                id_source_client: 2,
                id_source_contact: 2,
                id: Some(2),
            },
        ]
    }

    use super::*;

    pub fn insert_batch_to_mapping_client_source_db(
        connection: &mut DbConnection,
    ) -> Result<(), DieselError> {
        use self::schema::legacy_staging::staging_customer::dsl::*;
        let data = &vec![
            (
                id_source_client.eq(1),
                id_source_contact.eq(1),
                Some(id.eq(1)),
                id_shop.eq(1),
                m_pricelist_id.eq(1),
                name.eq("Test 1"),
                email.eq("test1@atest.com"),
                active.eq(true),
                is_xxa_centrale.eq(false),
                free_shipping_amount.eq(0),
                update_client.eq(chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                update_contact.eq(chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                is_synchronised.eq(true),
                has_error.eq(true),
                force_update.eq(false),
            ),
            (
                id_source_client.eq(2),
                id_source_contact.eq(2),
                Some(id.eq(2)),
                id_shop.eq(2),
                m_pricelist_id.eq(2),
                name.eq("Test 2"),
                email.eq("test2@atest.com"),
                active.eq(true),
                is_xxa_centrale.eq(false),
                free_shipping_amount.eq(0),
                update_client.eq(chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                update_contact.eq(chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                is_synchronised.eq(true),
                has_error.eq(true),
                force_update.eq(false),
            ),
            (
                id_source_client.eq(1),
                id_source_contact.eq(3),
                None,
                id_shop.eq(2),
                m_pricelist_id.eq(2),
                name.eq("Test 3"),
                email.eq("test3@atest.com"),
                active.eq(true),
                is_xxa_centrale.eq(false),
                free_shipping_amount.eq(0),
                update_client.eq(chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                update_contact.eq(chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                is_synchronised.eq(true),
                has_error.eq(true),
                force_update.eq(false),
            ),
        ];

        diesel::insert_into(staging_customer)
            .values(data)
            .execute(connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    pub fn read_mapping_client(connection: &mut DbConnection) -> Vec<MappingClientModel> {
        schema::target::mapping_client_contact::dsl::mapping_client_contact
            .load::<MappingClientModel>(connection)
            .expect("Error loading updated OrderModel")
    }

    #[test]
    fn test_upsert_to_insert() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        bench::mapping_client_model_fixture()[0]
            .upsert(&mut connection)
            .expect("Error upserting MappingClientModel");

        let query_result = schema::target::mapping_client_contact::dsl::mapping_client_contact
            .filter(
                schema::target::mapping_client_contact::id_customer
                    .eq(bench::mapping_client_model_fixture()[0].id_customer),
            )
            .load::<MappingClientModel>(&mut connection)
            .expect("Error loading inserted MappingClientModel");

        assert_eq!(query_result.len(), 1);
        assert_eq!(
            query_result[0].id_customer,
            bench::mapping_client_model_fixture()[0].id_customer
        );
        assert_eq!(
            query_result[0].idp_id_client,
            bench::mapping_client_model_fixture()[0].idp_id_client
        );
    }

    #[test]
    fn test_upsert_to_update() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        bench::mapping_client_model_fixture()[0]
            .upsert(&mut connection)
            .expect("Error upserting first MappingClientModel");

        MappingClientModel::new(bench::mapping_client_model_fixture()[0].id_customer, 2)
            .upsert(&mut connection)
            .expect("Error upserting second MappingClientModel");

        let query_result = schema::target::mapping_client_contact::dsl::mapping_client_contact
            .filter(
                schema::target::mapping_client_contact::id_customer
                    .eq(bench::mapping_client_model_fixture()[0].id_customer),
            )
            .load::<MappingClientModel>(&mut connection)
            .expect("Error loading upserted MappingClientModel");

        assert_eq!(query_result.len(), 1);
        assert_eq!(
            query_result[0].id_customer,
            bench::mapping_client_model_fixture()[0].id_customer
        );
        assert_eq!(query_result[0].idp_id_client, 2);
    }

    #[test]
    fn test_read_source() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_batch_to_mapping_client_source_db(&mut connection)
            .expect("Error inserting batch to mapping client source db");

        let result =
            MappingClientSource::select_all(&mut connection).expect("Error reading source");

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id_source_contact, 1);
        assert_eq!(result[1].id_source_contact, 2);
    }
}
