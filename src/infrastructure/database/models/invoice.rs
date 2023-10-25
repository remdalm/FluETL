use super::{CanUpsertModel, Model};
use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rust_decimal::Decimal;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq, Debug, Clone)]
#[diesel(table_name = schema::target::invoice_type_lang)]
// Seem not to work, probably because it is not a proper one to many relationship
// #[diesel(belongs_to(InvoiceModel, foreign_key = id_invoice_type))]
#[diesel(primary_key(id_invoice_type, id_lang))]
pub struct InvoiceLangModel {
    pub id_invoice_type: u32,
    pub id_lang: u32,
    pub name: String,
}

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
    pub id_invoice_type: u32,
    pub total_tax_excl: Decimal,
    pub total_tax_incl: Decimal,
}

impl Model for InvoiceModel {}
impl CanUpsertModel for InvoiceModel {
    fn upsert(&self, connection: &mut DbConnection) -> Result<(), DieselError> {
        super::upsert!(schema::target::invoice::table, self, connection)
    }
}

impl Model for (InvoiceModel, Vec<InvoiceLangModel>) {}
impl CanUpsertModel for (InvoiceModel, Vec<InvoiceLangModel>) {
    fn upsert(&self, connection: &mut DbConnection) -> Result<(), DieselError> {
        connection.transaction(|connection| {
            super::upsert!(schema::target::invoice::table, &self.0, connection)?;
            super::upsert!(
                schema::target::invoice_type_lang::table,
                &self.1,
                connection
            )
        })
    }
}

pub fn batch_upsert(
    models: &[(InvoiceModel, Vec<InvoiceLangModel>)],
    connection: &mut DbConnection,
) -> Result<(), DieselError> {
    let invoices: Vec<&InvoiceModel> = models.iter().map(|tuple| &tuple.0).collect();
    let invoice_langs: Vec<&InvoiceLangModel> =
        models.iter().flat_map(|tuple| tuple.1.iter()).collect();
    connection.transaction(|connection| {
        super::upsert!(schema::target::invoice::table, invoices, connection)?;
        super::upsert!(
            schema::target::invoice_type_lang::table,
            invoice_langs,
            connection
        )
    })
}

#[cfg(test)]
pub mod tests {
    use serial_test::serial;

    use crate::infrastructure::database::{
        connection::tests::{get_test_pooled_connection, reset_test_database},
        models::SingleRowInsertable,
    };
    // use diesel::debug_query;
    // use diesel::mysql::Mysql;

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
                id_invoice_type: 1,
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
                id_invoice_type: 2,
                total_tax_excl: Decimal::new(-30000, 2),
                total_tax_incl: Decimal::new(36000, 2),
            },
        ]
    }

    pub fn invoice_lang_model_fixtures() -> [Vec<InvoiceLangModel>; 3] {
        [
            vec![
                InvoiceLangModel {
                    id_invoice_type: 1,
                    id_lang: 1,
                    name: "Bottle".to_string(),
                },
                InvoiceLangModel {
                    id_invoice_type: 1,
                    id_lang: 2,
                    name: "Bouteille".to_string(),
                },
            ],
            vec![InvoiceLangModel {
                id_invoice_type: 2,
                id_lang: 1,
                name: "Plate".to_string(),
            }],
            vec![
                InvoiceLangModel {
                    id_invoice_type: 2,
                    id_lang: 1,
                    name: "Bottle".to_string(),
                },
                InvoiceLangModel {
                    id_invoice_type: 2,
                    id_lang: 2,
                    name: "Bouteille".to_string(),
                },
            ],
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

    pub fn read_invoice_types(
        connection: &mut DbConnection,
        invoice: &InvoiceModel,
    ) -> Vec<InvoiceLangModel> {
        use schema::target::invoice_type_lang::dsl::*;

        invoice_type_lang
            .filter(id_invoice_type.eq(&invoice.id_invoice_type))
            .load(connection)
            .expect("Error loading updated InvoiceModel")
        // let query = InvoiceLangModel::belonging_to(&invoice);

        // let debugged_query = debug_query::<Mysql, _>(&query);
        // let sql = debugged_query.to_string();

        // query
        //     .load(connection)
        //     .expect("Error loading updated InvoiceLangModel")
    }

    pub fn batch_tuple_fixtures() -> Vec<(InvoiceModel, Vec<InvoiceLangModel>)> {
        vec![
            (
                invoice_model_fixtures()[0].clone(),
                invoice_lang_model_fixtures()[0].clone(),
            ),
            (
                invoice_model_fixtures()[1].clone(),
                invoice_lang_model_fixtures()[1].clone(),
            ),
        ]
    }

    #[test]
    #[serial]
    fn test_upsert_invoice_when_no_conflict() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_invoice(&mut connection, true, &invoice_model_fixtures()[0])
            .expect("Failed to upsert invoice");

        let result = schema::target::invoice::dsl::invoice
            .filter(schema::target::invoice::id_invoice.eq(&invoice_model_fixtures()[0].id_invoice))
            .load::<InvoiceModel>(&mut connection)
            .expect("Error loading inserted InvoiceLangModel");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], invoice_model_fixtures()[0]);
    }

    #[test]
    #[serial]
    fn test_upsert_invoice_when_conflict() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let mut invoice_models = invoice_model_fixtures();

        insert_invoice(&mut connection, false, &invoice_models[0])
            .expect("Failed to insert invoice");

        invoice_models[0].total_tax_incl = Decimal::new(121, 2);

        insert_invoice(&mut connection, true, &invoice_models[0])
            .expect("Failed to upsert invoice");

        let result = schema::target::invoice::dsl::invoice
            .filter(schema::target::invoice::id_invoice.eq(1))
            .load::<InvoiceModel>(&mut connection)
            .expect("Error loading inserted/upserted InvoiceModel");

        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            InvoiceModel {
                total_tax_incl: Decimal::new(121, 2),
                ..invoice_model_fixtures()[0].clone()
            }
        );
    }

    #[test]
    #[serial]
    fn test_upsert_invoice_multiple_languages() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let invoice = &invoice_model_fixtures()[0];
        let langs = &invoice_lang_model_fixtures()[0];

        let model_with_langs = (invoice.clone(), langs.clone());

        model_with_langs
            .upsert(&mut connection)
            .expect("Failed to upsert invoice with multiple languages");

        // Check if the invoice is inserted
        let saved_invoice: Vec<InvoiceModel> = schema::target::invoice::dsl::invoice
            .filter(schema::target::invoice::id_invoice.eq(invoice.id_invoice))
            .load(&mut connection)
            .expect("Error loading invoice");

        assert_eq!(saved_invoice.len(), 1);
        assert_eq!(saved_invoice[0], *invoice);

        // Check if the languages are inserted
        let saved_langs: Vec<InvoiceLangModel> = read_invoice_types(&mut connection, invoice);

        assert_eq!(saved_langs.len(), langs.len());
        for lang in langs {
            assert!(saved_langs.contains(lang));
        }
    }

    #[test]
    #[serial]
    fn test_batch_upsert() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let invoices = batch_tuple_fixtures().clone();

        batch_upsert(&invoices, &mut connection).expect("Failed to batch upsert");

        let saved_invoices = read_invoices(&mut connection);
        assert_eq!(saved_invoices.len(), 2);
        assert!(saved_invoices.contains(&invoice_model_fixtures()[0]));
        assert!(saved_invoices.contains(&invoice_model_fixtures()[1]));

        assert!(read_invoice_types(&mut connection, &saved_invoices[0])
            .iter()
            .any(|invoice_type| invoice_lang_model_fixtures()[0].contains(invoice_type)));
        assert!(read_invoice_types(&mut connection, &saved_invoices[1])
            .iter()
            .any(|invoice_type| invoice_lang_model_fixtures()[1].contains(invoice_type)));
    }

    #[test]
    #[serial]
    fn test_batch_upsert_transaction_rollback() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let mut invoices = batch_tuple_fixtures().clone();
        invoices.push((
            InvoiceModel {
                id_invoice: 1, // this id will cause a foreign key violation or similar
                ..invoice_model_fixtures()[1].clone()
            },
            invoice_lang_model_fixtures()[0].clone(),
        ));

        let res = batch_upsert(&invoices, &mut connection);
        // We expect an error because one of the invoices has an invalid id
        assert!(res.is_err());

        // Check that no invoices were inserted due to transaction rollback
        let saved_invoices = read_invoices(&mut connection);
        assert_eq!(saved_invoices.len(), 0);
    }
}
