use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

use super::{SingleRowInsertable, SingleRowUpdatable};

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq)]
#[table_name = "schema::order"]
#[diesel(primary_key(id_order))]
pub struct OrderModel {
    pub id_order: u32,
    pub id_client: u32,
    pub order_ref: String,
    pub date: chrono::NaiveDateTime,
    pub order_status: Option<String>,
    pub delivery_status: Option<String>,
}

impl OrderModel {
    pub fn new(
        id_order: u32,
        id_client: u32,
        order_ref: String,
        date: chrono::NaiveDateTime,
    ) -> Self {
        OrderModel {
            id_order,
            id_client,
            order_ref,
            date,
            order_status: None,
            delivery_status: None,
        }
    }
}

impl SingleRowInsertable<schema::order::table, DbConnection> for OrderModel {
    fn target_client_table(&self) -> schema::order::table {
        schema::order::table
    }
}

impl SingleRowUpdatable<schema::order::table, DbConnection> for OrderModel {
    fn target_client_table(&self) -> schema::order::table {
        schema::order::table
    }
}

impl OrderModel {
    pub fn upsert(&self, connection: &mut DbConnection) -> Result<(), DieselError> {
        diesel::insert_into(schema::order::table)
            .values(self)
            .on_conflict(diesel::dsl::DuplicatedKeys)
            .do_update()
            .set(self)
            .execute(connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    // Used by benchmarks
    // Just to be convinced that the recommended way is far more optimised and should be used throughout the codebase
    pub fn homemade_upsert(&self, connection: &mut DbConnection) -> Result<(), DieselError> {
        let insert_result = self.insert(connection);
        if let Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) =
            insert_result
        {
            self.update(connection)
        } else {
            insert_result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        infrastructure::database::models::mapping_client::insert_mapping_client,
        tests::common::{get_test_pooled_connection, reset_test_database},
    };
    use diesel::result::{DatabaseErrorKind, Error as DieselError};

    fn insert_foreign_keys(connection: &mut DbConnection) -> Result<(), DieselError> {
        insert_mapping_client(connection)
    }

    pub fn insert_order(
        connection: &mut DbConnection,
        use_upsert: bool,
    ) -> Result<(), DieselError> {
        let new_order = OrderModel::new(1, 1, "Ref1".to_string(), chrono::Utc::now().naive_utc());
        if use_upsert {
            new_order.upsert(connection)
        } else {
            new_order.insert(connection)
        }
    }

    use super::*;
    #[test]
    fn test_insert_order() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let _ = insert_foreign_keys(&mut connection);

        let new_order = insert_order(&mut connection, false).expect("Failed to insert order");

        let result = schema::order::dsl::order
            .filter(schema::order::id_order.eq(1))
            .load::<OrderModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id_order, 1);
    }

    #[test]
    fn test_update_order() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");
        insert_order(&mut connection, false).expect("Failed to insert order");

        let mut fetched_orders = schema::order::dsl::order
            .filter(schema::order::id_order.eq(1))
            .load::<OrderModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        fetched_orders[0].order_ref = "Updated Ref1".to_string();

        fetched_orders[0]
            .update(&mut connection)
            .expect("Failed to update order");

        // Verify the order has been updated
        let retrieved_order = schema::order::dsl::order
            .filter(schema::order::id_order.eq(1))
            .load::<OrderModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(retrieved_order.len(), 1);
        assert_eq!(retrieved_order[0].order_ref, "Updated Ref1".to_string());
    }

    #[test]
    fn test_insert_duplicated_key() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");
        insert_order(&mut connection, false).expect("Failed to insert order");

        let duplicate = insert_order(&mut connection, false);

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

        insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");
        insert_order(&mut connection, true).expect("Failed to insert order by upsert function");

        let result = schema::order::dsl::order
            .filter(schema::order::id_order.eq(1))
            .load::<OrderModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id_order, 1);
    }

    #[test]
    fn test_upsert_order_when_conflit() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");
        insert_order(&mut connection, false).expect("Failed to insert order");
        insert_order(&mut connection, true).expect("Failed to upsert order");

        let result = schema::order::dsl::order
            .filter(schema::order::id_order.eq(1))
            .load::<OrderModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id_order, 1);
    }
}
