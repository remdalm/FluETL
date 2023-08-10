use crate::infrastructure::database::schema;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::MysqlConnection;

#[derive(Queryable, Identifiable, Insertable, AsChangeset)]
#[table_name = "schema::order"]
#[diesel(primary_key(id_order))]
pub struct Order {
    pub id_order: u32,
    pub id_client: u32,
    pub order_ref: String,
    pub date: chrono::NaiveDateTime,
    pub order_status: Option<String>,
    pub delivery_status: Option<String>,
}

impl Order {
    pub fn new(
        id_order: u32,
        id_client: u32,
        order_ref: String,
        date: chrono::NaiveDateTime,
    ) -> Self {
        Order {
            id_order,
            id_client,
            order_ref,
            date,
            order_status: None,
            delivery_status: None,
        }
    }

    pub fn insert(&self, connection: &mut MysqlConnection) -> Result<(), DieselError> {
        diesel::insert_into(schema::order::table)
            .values(self)
            .execute(connection)
            .map(|_| ())
    }

    pub fn update_order(
        &self,
        connection: &mut MysqlConnection,
        new_order: &Order,
    ) -> Result<(), DieselError> {
        let update_result = diesel::update(schema::order::dsl::order.find(self.id_order))
            .set(new_order)
            .execute(connection);

        update_result.map(|_| ()).map_err(|e| e.into())
    }
}
