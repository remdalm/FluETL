use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use diesel::prelude::*;

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

#[cfg(test)]
mod tests {
    use crate::{
        infrastructure::database::models::mapping_client::insert_mapping_client,
        tests::common::{get_test_pooled_connection, reset_test_database},
    };
    use diesel::result::Error as DieselError;

    fn insert_foreign_keys(connection: &mut DbConnection) -> Result<(), DieselError> {
        insert_mapping_client(connection)
    }

    pub fn insert_order(connection: &mut DbConnection) -> Result<(), DieselError> {
        let new_order = OrderModel::new(1, 1, "Ref1".to_string(), chrono::Utc::now().naive_utc());
        new_order.insert(connection)
    }

    use super::*;
    #[test]
    fn test_insert_order() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let _ = insert_foreign_keys(&mut connection);

        let new_order = insert_order(&mut connection).expect("Failed to insert order");

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

        let _ = insert_foreign_keys(&mut connection);
        let _ = insert_order(&mut connection).expect("Failed to insert order");

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
}
