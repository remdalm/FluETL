use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use super::{CanUpsertModel, Model};

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq, Debug, Clone)]
#[diesel(table_name = schema::target::order_line)]
#[diesel(primary_key(id_order_line))]
pub struct OrderLineModel {
    pub id_order_line: u32,
    pub id_order: u32,
    pub product_ref: String,
    pub product_name: Option<String>,
    pub qty_ordered: u32,
    pub qty_reserved: u32,
    pub qty_delivered: u32,
    pub due_date: NaiveDate,
}

impl Model for OrderLineModel {}
impl CanUpsertModel for OrderLineModel {
    fn upsert(&self, connection: &mut DbConnection) -> Result<(), DieselError> {
        diesel::insert_into(schema::target::order_line::table)
            .values(self)
            .on_conflict(diesel::dsl::DuplicatedKeys)
            .do_update()
            .set(self)
            .execute(connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
pub mod tests {
    use diesel::result::DatabaseErrorKind;

    use crate::{
        fixtures::{order_line_model_fixtures, order_model_fixtures},
        infrastructure::database::{
            connection::tests::{get_test_pooled_connection, reset_test_database},
            models::{order::tests::insert_order, SingleRowInsertable},
        },
    };

    use super::*;

    impl SingleRowInsertable<schema::target::order_line::table, DbConnection> for OrderLineModel {
        fn target_client_table(&self) -> schema::target::order_line::table {
            schema::target::order_line::table
        }
    }

    pub fn insert_foreign_keys(connection: &mut DbConnection) -> Result<(), DieselError> {
        insert_order(connection, false, &order_model_fixtures()[0])
    }

    pub fn insert_order_line(
        connection: &mut DbConnection,
        use_upsert: bool,
        new_order_line: &OrderLineModel,
    ) -> Result<(), DieselError> {
        if use_upsert {
            new_order_line.upsert(connection)
        } else {
            new_order_line.insert(connection)
        }
    }

    pub fn read_order_lines(connection: &mut DbConnection) -> Vec<OrderLineModel> {
        schema::target::order_line::dsl::order_line
            .load::<OrderLineModel>(connection)
            .expect("Error loading updated OrderLineModel")
    }

    #[test]
    fn test_upsert_order_line_when_no_conflit() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");
        insert_order_line(&mut connection, false, &order_line_model_fixtures()[0])
            .expect("Failed to upsert order line");

        let result = schema::target::order_line::dsl::order_line
            .filter(
                schema::target::order_line::id_order_line
                    .eq(&order_line_model_fixtures()[0].id_order_line),
            )
            .load::<OrderLineModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], order_line_model_fixtures()[0]);
    }

    #[test]
    fn test_upsert_order_line_when_conflit() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let mut order_line_models = order_line_model_fixtures();

        insert_foreign_keys(&mut connection).expect("Failed to insert foreign keys");
        insert_order_line(&mut connection, false, &order_line_models[0])
            .expect("Failed to insert order");

        order_line_models[0].qty_delivered = 10;

        insert_order_line(&mut connection, true, &order_line_models[0])
            .expect("Failed to upsert order");

        let result = schema::target::order_line::dsl::order_line
            .filter(schema::target::order_line::id_order_line.eq(1))
            .load::<OrderLineModel>(&mut connection)
            .expect("Error loading inserted OrderModel");

        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            OrderLineModel {
                qty_delivered: 10,
                ..order_line_model_fixtures()[0].clone()
            }
        );
    }

    #[test]
    fn test_upsert_order_line_when_no_() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let result = insert_order_line(&mut connection, false, &order_line_model_fixtures()[0]);

        assert!(result.is_err_and(|e| match e {
            DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) => true,
            _ => false,
        }));
    }
}
