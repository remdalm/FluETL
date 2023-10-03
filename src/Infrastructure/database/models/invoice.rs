use super::{CanUpsertModel, Model};
use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rust_decimal::Decimal;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq, Debug, Clone)]
#[diesel(table_name = schema::target::invoice)]
#[diesel(primary_key(id_invoice))]
pub struct InvoiceModel {
    pub id_invoice: u32,
    pub id_client: u32,
    pub client_name: Option<String>,
    pub invoice_ref: String,
    pub date: NaiveDate,
    pub file_name: Option<String>,
    pub po_ref: Option<String>,
    pub type_: String,
    pub total_tax_excl: Decimal,
    pub total_tax_incl: Decimal,
}

impl Model for InvoiceModel {}
impl CanUpsertModel for InvoiceModel {
    fn upsert(&self, connection: &mut DbConnection) -> Result<(), DieselError> {
        diesel::insert_into(schema::target::invoice::table)
            .values(self)
            .on_conflict(diesel::dsl::DuplicatedKeys)
            .do_update()
            .set(self)
            .execute(connection)
            .map(|_| ())
    }
}

pub fn batch_upsert(
    models: &[InvoiceModel],
    connection: &mut DbConnection,
) -> Result<(), DieselError> {
    let query = diesel::replace_into(schema::target::invoice::table).values(models);

    query.execute(connection).map(|_| ())
}

#[cfg(test)]
pub mod tests {
    use crate::infrastructure::database::{
        connection::tests::{get_test_pooled_connection, reset_test_database},
        models::SingleRowInsertable,
    };

    use super::*;
    pub fn invoice_model_fixtures() -> [InvoiceModel; 2] {
        [
            InvoiceModel {
                id_invoice: 1,
                id_client: 1,
                client_name: Some("Client 1".to_string()),
                invoice_ref: "INV-1".to_string(),
                file_name: Some("INV-1.pdf".to_string()),
                date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                po_ref: Some("PO-1".to_string()),
                type_: "Invoice 123".to_string(),
                total_tax_excl: Decimal::new(10000, 2),
                total_tax_incl: Decimal::new(12000, 2),
            },
            InvoiceModel {
                id_invoice: 3,
                id_client: 1,
                client_name: Some("Client 1".to_string()),
                invoice_ref: "INV-3".to_string(),
                file_name: Some("INV-3.pdf".to_string()),
                date: NaiveDate::from_ymd_opt(2020, 1, 3).unwrap(),
                po_ref: None,
                type_: "Invoice 456".to_string(),
                total_tax_excl: Decimal::new(-30000, 2),
                total_tax_incl: Decimal::new(36000, 2),
            },
        ]
    }

    impl SingleRowInsertable<schema::target::invoice::table, DbConnection> for InvoiceModel {
        fn target_client_table(&self) -> schema::target::invoice::table {
            schema::target::invoice::table
        }
    }

    pub fn insert_invoice(
        connection: &mut DbConnection,
        use_upsert: bool,
        new_invoice: &InvoiceModel,
    ) -> Result<(), DieselError> {
        if use_upsert {
            new_invoice.upsert(connection)
        } else {
            new_invoice.insert(connection)
        }
    }

    pub fn read_invoices(connection: &mut DbConnection) -> Vec<InvoiceModel> {
        schema::target::invoice::dsl::invoice
            .load::<InvoiceModel>(connection)
            .expect("Error loading updated InvoiceModel")
    }

    #[test]
    fn test_upsert_invoice_when_no_conflit() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_invoice(&mut connection, true, &invoice_model_fixtures()[0])
            .expect("Failed to upsert delivery slip");

        let result = schema::target::invoice::dsl::invoice
            .filter(schema::target::invoice::id_invoice.eq(&invoice_model_fixtures()[0].id_invoice))
            .load::<InvoiceModel>(&mut connection)
            .expect("Error loading inserted DeloverySlipModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], invoice_model_fixtures()[0]);
    }

    #[test]
    fn test_upsert_invoice_when_conflit() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let mut invoice_models = invoice_model_fixtures();

        insert_invoice(&mut connection, false, &invoice_models[0]).expect("Failed to insert order");

        invoice_models[0].total_tax_incl = Decimal::new(121, 2);

        insert_invoice(&mut connection, true, &invoice_models[0]).expect("Failed to upsert order");

        let result = schema::target::invoice::dsl::invoice
            .filter(schema::target::invoice::id_invoice.eq(1))
            .load::<InvoiceModel>(&mut connection)
            .expect("Error loading inserted/upserted DeloverySlipModel");

        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            InvoiceModel {
                total_tax_incl: Decimal::new(121, 2),
                ..invoice_model_fixtures()[0].clone()
            }
        );
    }
}
