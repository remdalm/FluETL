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
    pub qty_ordered: u32,
    pub qty_reserved: u32,
    pub qty_delivered: u32,
    pub due_date: Option<NaiveDate>,
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
    }
}

pub fn batch_upsert(
    models: &[OrderLineModel],
    connection: &mut DbConnection,
) -> Result<(), DieselError> {
    let query = diesel::replace_into(schema::target::order_line::table).values(models);

    query.execute(connection).map(|_| ())
}

#[cfg(test)]
pub mod tests {
    use diesel::result::DatabaseErrorKind;

    use crate::infrastructure::database::{
        connection::tests::{get_test_pooled_connection, reset_test_database},
        models::{
            order::{bench::order_model_fixtures, tests::insert_order},
            SingleRowInsertable,
        },
    };

    use super::*;

    pub fn order_line_model_fixtures() -> [OrderLineModel; 3] {
        [
            OrderLineModel {
                id_order_line: 1,
                id_order: 1,
                product_ref: "ItemRef1".to_string(),
                qty_ordered: 10,
                qty_reserved: 5,
                qty_delivered: 5,
                due_date: Some(NaiveDate::from_ymd_opt(2023, 8, 1).unwrap()),
            },
            OrderLineModel {
                id_order_line: 2,
                id_order: 1,
                product_ref: "ItemRef2".to_string(),
                qty_ordered: 20,
                qty_reserved: 10,
                qty_delivered: 10,
                due_date: Some(NaiveDate::from_ymd_opt(2023, 8, 2).unwrap()),
            },
            OrderLineModel {
                id_order_line: 3,
                id_order: 2,
                product_ref: "ItemRef3".to_string(),
                qty_ordered: 30,
                qty_reserved: 15,
                qty_delivered: 15,
                due_date: None,
            },
        ]
    }

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
        matches!(
            result,
            Err(DieselError::DatabaseError(
                DatabaseErrorKind::ForeignKeyViolation,
                _
            ))
        );
    }
}
