use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use super::{CanUpsertModel, Model, SingleRowInsertable, SingleRowUpdatable};

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq, Debug)]
#[diesel(table_name = schema::target::order)]
#[diesel(primary_key(id_order))]
pub struct OrderModel {
    pub id_order: u32,
    pub id_client: u32,
    pub order_ref: String,
    pub date: chrono::NaiveDateTime,
    pub order_status: Option<String>,
    pub delivery_status: Option<String>,
}

impl Model for OrderModel {}
impl CanUpsertModel for OrderModel {
    fn upsert(&self, connection: &mut DbConnection) -> Result<(), DieselError> {
        diesel::insert_into(schema::target::order::table)
            .values(self)
            .on_conflict(diesel::dsl::DuplicatedKeys)
            .do_update()
            .set(self)
            .execute(connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }
}

impl OrderModel {
    pub fn new(
        id_order: u32,
        id_client: u32,
        order_ref: String,
        date: chrono::NaiveDateTime,
        order_status: Option<String>,
        delivery_status: Option<String>,
    ) -> Self {
        OrderModel {
            id_order,
            id_client,
            order_ref,
            date,
            order_status: order_status,
            delivery_status: delivery_status,
        }
    }
}

impl SingleRowInsertable<schema::target::order::table, DbConnection> for OrderModel {
    fn target_client_table(&self) -> schema::target::order::table {
        schema::target::order::table
    }
}

impl SingleRowUpdatable<schema::target::order::table, DbConnection> for OrderModel {
    fn target_client_table(&self) -> schema::target::order::table {
        schema::target::order::table
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        fixtures::order_model_fixtures,
        infrastructure::database::connection::tests::{
            get_test_pooled_connection, reset_test_database,
        },
        // infrastructure::database::models::mapping_client::insert_mapping_client,
    };
    use diesel::result::{DatabaseErrorKind, Error as DieselError};

    // pub fn insert_foreign_keys(connection: &mut DbConnection) -> Result<(), DieselError> {
    //     insert_mapping_client(connection)
    // }

    pub fn insert_order(
        connection: &mut DbConnection,
        use_upsert: bool,
        new_order: &OrderModel,
    ) -> Result<(), DieselError> {
        if use_upsert {
            new_order.upsert(connection)
        } else {
            new_order.insert(connection)
        }
    }

    pub fn read_orders(connection: &mut DbConnection) -> Vec<OrderModel> {
        schema::target::order::dsl::order
            .load::<OrderModel>(connection)
            .expect("Error loading updated OrderModel")
    }

    use super::*;
    #[test]
    fn test_insert_order() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // let _ = insert_foreign_keys(&mut connection);

        insert_order(&mut connection, false, &order_model_fixtures()[0])
            .expect("Failed to insert order");

        let result = schema::target::order::dsl::order
            .filter(schema::target::order::id_order.eq(1))
            .load::<OrderModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id_order, 1);
    }

    #[test]
    fn test_update_order() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");
        insert_order(&mut connection, false, &order_model_fixtures()[0])
            .expect("Failed to insert order");

        let mut fetched_orders = schema::target::order::dsl::order
            .filter(schema::target::order::id_order.eq(1))
            .load::<OrderModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        fetched_orders[0].order_ref = "Updated Ref1".to_string();

        fetched_orders[0]
            .update(&mut connection)
            .expect("Failed to update order");

        // Verify the order has been updated
        let retrieved_order = schema::target::order::dsl::order
            .filter(schema::target::order::id_order.eq(1))
            .load::<OrderModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(retrieved_order.len(), 1);
        assert_eq!(retrieved_order[0].order_ref, "Updated Ref1".to_string());
    }

    #[test]
    fn test_insert_duplicated_key() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");
        insert_order(&mut connection, false, &order_model_fixtures()[0])
            .expect("Failed to insert order");

        let duplicate = insert_order(&mut connection, false, &order_model_fixtures()[0]);

        if let Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) = duplicate {
            // Expected
        } else {
            panic!(
                "{}",
                format!("Expected duplicate key error, got {:?}", duplicate)
            );
        }
    }

    #[test]
    fn test_upsert_order_when_no_conflit() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");
        insert_order(&mut connection, true, &order_model_fixtures()[0])
            .expect("Failed to insert order by upsert function");

        let result = schema::target::order::dsl::order
            .filter(schema::target::order::id_order.eq(1))
            .load::<OrderModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id_order, 1);
    }

    #[test]
    fn test_upsert_order_when_conflit() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");
        insert_order(&mut connection, false, &order_model_fixtures()[0])
            .expect("Failed to insert order");
        insert_order(&mut connection, true, &order_model_fixtures()[0])
            .expect("Failed to upsert order");

        let result = schema::target::order::dsl::order
            .filter(schema::target::order::id_order.eq(1))
            .load::<OrderModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id_order, 1);
    }
}
