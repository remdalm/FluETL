use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use diesel::prelude::*;

use super::ModelOps;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq)]
#[table_name = "schema::mapping_client_contact"]
#[diesel(primary_key(idp_id_client))]
pub struct MappingClientModel {
    pub idp_id_client: u32,
    pub ps_id_customer: u32,
}

impl ModelOps<schema::mapping_client_contact::table, DbConnection> for MappingClientModel {
    // fn insert(
    //     &self,
    //     connection: &mut DbConnection,
    //     table: Self::TargetTable,
    // ) -> Result<(), DieselError> {
    //     diesel::insert_into(table)
    //         .values(self)
    //         .execute(connection)
    //         .map(|_| ())
    //         .map_err(|e| e.into())
    // }

    // fn update(
    //     &self,
    //     connection: &mut DbConnection,
    //     table: Self::TargetTable,
    // ) -> Result<(), DieselError> {
    //     diesel::update(table.find(self.idp_id_client))
    //         .set(self)
    //         .execute(connection)
    //         .map(|_| ())
    //         .map_err(|e| e.into())
    // }
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
    #[test]
    fn test_insert_order() {
        let mut connection = get_test_pooled_connection();
        setup_test_database(&mut connection);
        // Insert an order
        let new_order = MappingClientModel::new(1, 1);
        new_order
            .insert(&mut connection)
            .expect("Failed to insert order");

        let result = schema::mapping_client_contact::dsl::mapping_client_contact
            .filter(schema::mapping_client_contact::idp_id_client.eq(1))
            .load::<MappingClientModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].idp_id_client, 1);

        //Clean up the test database
        teardown_test_database(&mut connection);
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
