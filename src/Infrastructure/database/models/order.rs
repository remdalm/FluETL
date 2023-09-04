use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use super::{CanUpsertModel, Model, SingleRowInsertable, SingleRowUpdatable};

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq, Debug, Clone)]
#[diesel(table_name = schema::target::order)]
#[diesel(primary_key(id_order))]
pub struct OrderModel {
    pub id_order: u32,
    pub id_client: u32,
    pub client_name: Option<String>,
    pub order_ref: String,
    pub date: chrono::NaiveDateTime,
    pub po_ref: Option<String>,
    pub origin: Option<String>,
    pub completion: Option<u32>,
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

// pub(crate) trait CanFetchOrderModel: HasTargetConnection {
//     fn fetch_order(&self, order_id: &u32) -> Result<OrderModel, InfrastructureError> {
//         OrderModel::select_by_id(&mut self.get_pooled_connection(), order_id)
//             .map_err(|e| InfrastructureError::DatabaseError(e))
//     }
// }

impl OrderModel {
    pub fn select_by_id(
        connection: &mut DbConnection,
        order_id: &u32,
    ) -> Result<Self, DieselError> {
        use self::schema::target::order::dsl::*;
        order
            .filter(id_order.eq(order_id))
            .first(connection)
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        fixtures::order_model_fixtures,
        infrastructure::database::connection::tests::{
            get_test_pooled_connection, reset_test_database,
        },
    };
    use diesel::result::{DatabaseErrorKind, Error as DieselError};

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
    fn test_select_order_by_id() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_order(&mut connection, false, &order_model_fixtures()[0])
            .expect("Failed to insert order");
        insert_order(&mut connection, false, &order_model_fixtures()[1])
            .expect("Failed to insert order");

        let result1 =
            OrderModel::select_by_id(&mut connection, &order_model_fixtures()[0].id_order)
                .expect("Failed to select order by id");
        let result2 =
            OrderModel::select_by_id(&mut connection, &order_model_fixtures()[1].id_order)
                .expect("Failed to select order by id");

        assert_eq!(result1, order_model_fixtures()[0]);
        assert_eq!(result2, order_model_fixtures()[1]);
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
