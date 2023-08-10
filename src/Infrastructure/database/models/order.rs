use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::MysqlConnection;

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

    pub fn insert(&self, connection: &mut DbConnection) -> Result<(), DieselError> {
        diesel::insert_into(schema::order::table)
            .values(self)
            .execute(connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    pub fn update_order(
        &self,
        connection: &mut DbConnection,
        new_order: &OrderModel,
    ) -> Result<(), DieselError> {
        diesel::update(schema::order::dsl::order.find(self.id_order))
            .set(new_order)
            .execute(connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::common::{
        get_test_pooled_connection, setup_test_database, teardown_test_database,
    };

    use super::*;
    #[test]
    fn test_insert_order() {
        let mut connection = get_test_pooled_connection();
        setup_test_database(&mut connection);
        // Insert an order
        let new_order = OrderModel::new(1, 1, "Ref1".to_string(), chrono::Utc::now().naive_utc());
        new_order
            .insert(&mut connection)
            .expect("Failed to insert order");

        let result = schema::order::dsl::order
            .filter(schema::order::id_order.eq(1))
            .load::<OrderModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id_order, 1);

        // Clean up the test database
        //teardown_test_database(connection);
    }

    // #[test]
    // fn test_update_order() {
    //     // Set up the test database
    //     let connection = setup_test_database();

    //     // Insert an order
    //     let new_order = NewOrderModel::new(1, "12345".to_string(), chrono::Utc::now().naive_utc());
    //     new_order
    //         .insert(&connection)
    //         .expect("Failed to insert order");

    //     // Update the order
    //     let order = OrderModel::find_by_order_ref(&connection, "12345".to_string())
    //         .expect("Failed to find order");
    //     let updated_order =
    //         OrderModel::update_order(&connection, &order, Some("67890".to_string()), None, None)
    //             .expect("Failed to update order");

    //     // Verify the order has been updated
    //     let retrieved_order = OrderModel::find_by_order_ref(&connection, "67890".to_string())
    //         .expect("Failed to find updated order");
    //     assert_eq!(retrieved_order.order_ref, "67890");

    //     // Clean up the test database
    //     teardown_test_database(&connection);
    // }
}
