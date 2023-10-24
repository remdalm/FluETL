use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use super::{CanUpsertModel, Model};

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq, Debug, Clone)]
#[diesel(table_name = schema::target::delivery_slip)]
#[diesel(primary_key(id_delivery_slip))]
pub struct DeliverySlipModel {
    pub id_delivery_slip: u32,
    pub id_client: u32,
    pub reference: String,
    pub shipping_date: Option<NaiveDate>,
    pub po_ref: Option<String>,
    pub carrier_name: Option<String>,
    pub status: Option<String>,
    pub tracking_number: Option<String>,
    pub tracking_link: Option<String>,
}

impl Model for DeliverySlipModel {}
impl CanUpsertModel for DeliverySlipModel {
    fn upsert(&self, connection: &mut DbConnection) -> Result<(), DieselError> {
        diesel::insert_into(schema::target::delivery_slip::table)
            .values(self)
            .on_conflict(diesel::dsl::DuplicatedKeys)
            .do_update()
            .set(self)
            .execute(connection)
            .map(|_| ())
    }
}

pub fn batch_upsert(
    models: &[DeliverySlipModel],
    connection: &mut DbConnection,
) -> Result<(), DieselError> {
    let query = diesel::replace_into(schema::target::delivery_slip::table).values(models);

    query.execute(connection).map(|_| ())
}

#[cfg(test)]
pub mod tests {
    use serial_test::serial;

    use crate::infrastructure::database::{
        connection::tests::{get_test_pooled_connection, reset_test_database},
        models::SingleRowInsertable,
    };

    use super::*;
    pub fn delivery_slip_model_fixtures() -> [DeliverySlipModel; 3] {
        [
            DeliverySlipModel {
                id_delivery_slip: 1,
                id_client: 1,
                reference: "Doc1".to_string(),
                shipping_date: Some(NaiveDate::from_ymd_opt(2023, 8, 1).unwrap()),
                po_ref: Some("PoRef1".to_string()),
                carrier_name: Some("Carrier1".to_string()),
                status: Some("CO".to_string()),
                tracking_number: Some("TrackingNo1".to_string()),
                tracking_link: Some("https://tracking1.com/123".to_string()),
            },
            DeliverySlipModel {
                id_delivery_slip: 2,
                id_client: 2,
                reference: "Doc2".to_string(),
                shipping_date: Some(NaiveDate::from_ymd_opt(2023, 8, 2).unwrap()),
                po_ref: Some("PoRef2".to_string()),
                carrier_name: Some("Carrier2".to_string()),
                status: Some("IN".to_string()),
                tracking_number: Some("TrackingNo2".to_string()),
                tracking_link: None,
            },
            DeliverySlipModel {
                id_delivery_slip: 3,
                id_client: 1,
                reference: "Doc3".to_string(),
                shipping_date: None,
                po_ref: None,
                carrier_name: None,
                status: None,
                tracking_number: None,
                tracking_link: None,
            },
        ]
    }

    impl SingleRowInsertable<schema::target::delivery_slip::table, DbConnection> for DeliverySlipModel {
        fn target_client_table(&self) -> schema::target::delivery_slip::table {
            schema::target::delivery_slip::table
        }
    }

    pub fn insert_delivery_slip(
        connection: &mut DbConnection,
        use_upsert: bool,
        new_delivery_slip: &DeliverySlipModel,
    ) -> Result<(), DieselError> {
        if use_upsert {
            new_delivery_slip.upsert(connection)
        } else {
            new_delivery_slip.insert(connection)
        }
    }

    pub fn read_delivery_slips(connection: &mut DbConnection) -> Vec<DeliverySlipModel> {
        schema::target::delivery_slip::dsl::delivery_slip
            .load::<DeliverySlipModel>(connection)
            .expect("Error loading updated DeliverySlipModel")
    }

    #[test]
    #[serial]
    fn test_upsert_delivery_slip_when_no_conflict() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_delivery_slip(&mut connection, true, &delivery_slip_model_fixtures()[0])
            .expect("Failed to upsert delivery slip");

        let result = schema::target::delivery_slip::dsl::delivery_slip
            .filter(
                schema::target::delivery_slip::id_delivery_slip
                    .eq(&delivery_slip_model_fixtures()[0].id_delivery_slip),
            )
            .load::<DeliverySlipModel>(&mut connection)
            .expect("Error loading inserted DeloverySlipModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], delivery_slip_model_fixtures()[0]);
    }

    #[test]
    #[serial]
    fn test_upsert_delivery_slip_when_conflict() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let mut delivery_slip_models = delivery_slip_model_fixtures();

        insert_delivery_slip(&mut connection, false, &delivery_slip_models[0])
            .expect("Failed to insert order");

        delivery_slip_models[0].reference = "new reference".to_string();

        insert_delivery_slip(&mut connection, true, &delivery_slip_models[0])
            .expect("Failed to upsert order");

        let result = schema::target::delivery_slip::dsl::delivery_slip
            .filter(schema::target::delivery_slip::id_delivery_slip.eq(1))
            .load::<DeliverySlipModel>(&mut connection)
            .expect("Error loading inserted/upserted DeloverySlipModel");

        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            DeliverySlipModel {
                reference: "new reference".to_string(),
                ..delivery_slip_model_fixtures()[0].clone()
            }
        );
    }
}
