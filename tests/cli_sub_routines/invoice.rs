use chrono::NaiveDate;
use diesel::{sql_query, QueryableByName, RunQueryDsl};
use serial_test::serial;
use std::ops::Range;
use std::process::Command;

use fluetl::infrastructure::database::connection::DbConnection;

use crate::reset_test_database;
use crate::setup_database_connection;
use crate::{insert_raw_sql, panic_if_stdout_contains_error};

// ****************** //
// test fluetl import invoice --env-file=.env.test
// ****************** //
#[test]
#[serial]
fn import_invoice_once() {
    import_invoice(0..1);
}

#[test]
#[serial]
fn import_invoice_10_times() {
    import_invoice(0..10);
}

fn import_invoice(repeat: Range<i32>) {
    // Arrange
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    insert_raw_sql("tests/fixtures/languages.sql", &mut connection)
        .expect("Failed to insert languages.sql fixture file");

    // Import invoice:
    Command::new("target/debug/fluetl")
        .args(["import", "invoice", "--env-file=.env.test"])
        .output()
        .expect("Failed to execute command");

    // Result
    for _ in repeat {
        let output = Command::new("target/debug/fluetl")
            .args(["import", "invoice", "--env-file=.env.test"])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            panic_if_stdout_contains_error(&stdout);
            println!("Command executed successfully:\n{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Command failed:\n{}", stderr);
        }
    }

    assert_invoice_results(&mut connection);
}

// ****************** //
// test fluetl import invoice -b -s2 --env-file=.env.test
// ****************** //
#[test]
#[serial]
fn import_invoice_batch() {
    // Arrange
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    insert_raw_sql("tests/fixtures/languages.sql", &mut connection)
        .expect("Failed to insert languages.sql fixture file");

    // Import sinvoice:
    Command::new("target/debug/fluetl")
        .args(["import", "invoice", "--env-file=.env.test"])
        .output()
        .expect("Failed to execute command");

    let output = Command::new("target/debug/fluetl")
        .args(["import", "invoice", "-b", "-s2", "--env-file=.env.test"])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic_if_stdout_contains_error(&stdout);
        println!("Command executed successfully:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed:\n{}", stderr);
    }
    assert_invoice_results(&mut connection);
}

fn assert_invoice_results(connection: &mut DbConnection) {
    // Assert Invoice Table
    let invoice_query_results = sql_query("SELECT * FROM `invoice`")
        .load::<InvoicePlaceholder>(connection)
        .expect("Failed to query invoice table");

    let invoice_query_result_sample_1 =
        sql_query("SELECT * FROM `invoice` WHERE id_invoice = 1000060")
            .load::<InvoicePlaceholder>(connection)
            .expect("Failed to query invoice table");

    let invoice_query_result_sample_2 =
        sql_query("SELECT * FROM `invoice` WHERE id_invoice = 1000058")
            .load::<InvoicePlaceholder>(connection)
            .expect("Failed to query invoice table");

    let invoice_query_result_sample_3 =
        sql_query("SELECT * FROM `invoice` WHERE id_invoice = 1000053")
            .load::<InvoicePlaceholder>(connection)
            .expect("Failed to query invoice table");

    assert_eq!(invoice_query_results.len(), 4); // 1 must fail because of validation error
    assert_eq!(
        invoice_query_result_sample_1[0],
        InvoicePlaceholder {
            id_invoice: 1000060,
            id_client: 1012271,
            client_name: Some("CLIENT NAME 1".to_string()),
            invoice_ref: "A1000003".to_string(),
            date: NaiveDate::from_ymd_opt(2020, 11, 18).unwrap(),
            file_name: Some("690156201118A1000003209.pdf".to_string()),
            po_ref: Some("WEB143".to_string()),
            total_tax_excl: "18.54".to_string(),
            total_tax_incl: "22.25".to_string()
        }
    );

    assert_eq!(
        invoice_query_result_sample_2[0],
        InvoicePlaceholder {
            id_invoice: 1000058,
            id_client: 1009721,
            client_name: Some("CLIENT NAME 2 &é\"'(!ç".to_string()),
            invoice_ref: "A1000001".to_string(),
            date: NaiveDate::from_ymd_opt(2020, 11, 18).unwrap(),
            file_name: Some("666849201118A1000001209.pdf".to_string()),
            po_ref: Some("W043783".to_string()),
            total_tax_excl: "36.88".to_string(),
            total_tax_incl: "44.26".to_string()
        }
    );

    assert_eq!(
        invoice_query_result_sample_3[0],
        InvoicePlaceholder {
            id_invoice: 1000053,
            id_client: 1009287,
            client_name: None,
            invoice_ref: "FC000001".to_string(),
            date: NaiveDate::from_ymd_opt(2020, 11, 4).unwrap(),
            file_name: None,
            po_ref: None,
            total_tax_excl: "-54.09".to_string(),
            total_tax_incl: "-64.91".to_string()
        }
    );

    // Assert Invoice Lang Table
    let invoice_type_query_results = sql_query("SELECT * FROM `invoice_lang`")
        .load::<InvoiceTypePlaceholder>(connection)
        .expect("Failed to query invoice_lang table");

    let invoice_type_query_result_sample_1 =
        sql_query("SELECT * FROM `invoice_lang` WHERE id_invoice = 1000060")
            .load::<InvoiceTypePlaceholder>(connection)
            .expect("Failed to query invoice_lang table");

    // id 1000060 => 2 languages, 1000058 => 2 , 1000056 => 1 , 1000055 => Invalid, 1000053 => 2
    assert_eq!(invoice_type_query_results.len(), 7);
    assert_eq!(
        invoice_type_query_result_sample_1,
        vec![
            InvoiceTypePlaceholder {
                id_invoice: 1000060,
                id_lang: 1,
                type_name: "Credit Note - Entry error".to_string()
            },
            InvoiceTypePlaceholder {
                id_invoice: 1000060,
                id_lang: 2,
                type_name: "Avoir pour erreur de saisie".to_string()
            },
        ]
    );
}

#[derive(QueryableByName, Debug, PartialEq)]
struct InvoicePlaceholder {
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_invoice: u32,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_client: u32,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub client_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub invoice_ref: String,
    #[diesel(sql_type = diesel::sql_types::Date)]
    pub date: chrono::NaiveDate,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub file_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub po_ref: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub total_tax_excl: String,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub total_tax_incl: String,
}

#[derive(QueryableByName, Debug, PartialEq)]
struct InvoiceTypePlaceholder {
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_invoice: u32,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_lang: u32,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub type_name: String,
}
