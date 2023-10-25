use chrono::NaiveDate;
use diesel::{sql_query, QueryableByName, RunQueryDsl};
use fluetl::infrastructure::database::connection::DbConnection;
use serial_test::serial;
use std::ops::Range;
use std::process::Command;

use crate::setup_database_connection;
use crate::{panic_if_stdout_contains_error, reset_test_database};

// ****************** //
// test fluetl import delivery-slip --env-file=.env.test
// ****************** //
#[test]
#[serial]
fn import_delivery_slip_once() {
    import_delivery_slip(0..1);
}

#[test]
#[serial]
fn import_delivery_slip_10_times() {
    import_delivery_slip(0..10);
}

fn import_delivery_slip(repeat: Range<i32>) {
    // Arrange
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    // Result
    for _ in repeat {
        let output = Command::new("target/debug/fluetl")
            .args(["import", "delivery-slip", "--env-file=.env.test"])
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

    assert_delivery_slip_table(&mut connection);
}
// ****************** //
// test fluetl import delivery-slip -b -s2 --env-file=.env.test
// ****************** //
#[test]
#[serial]
fn import_delivery_slip_batch() {
    // Arrange
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    let output = Command::new("target/debug/fluetl")
        .args([
            "import",
            "delivery-slip",
            "-b",
            "-s2",
            "--env-file=.env.test",
        ])
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

    assert_delivery_slip_table(&mut connection);
}

fn assert_delivery_slip_table(connection: &mut DbConnection) {
    // Assert
    let query_results = sql_query("SELECT * FROM `delivery_slip`")
        .load::<DeliverySlipPlaceholder>(connection)
        .expect("Failed to query delivery_slip table");

    let query_result_sample_1 =
        sql_query("SELECT * FROM `delivery_slip` WHERE id_delivery_slip = 1011124")
            .load::<DeliverySlipPlaceholder>(connection)
            .expect("Failed to query delivery_slip table");

    let query_result_sample_2 =
        sql_query("SELECT * FROM `delivery_slip` WHERE id_delivery_slip = 1012515")
            .load::<DeliverySlipPlaceholder>(connection)
            .expect("Failed to query delivery_slip table");

    let query_result_sample_3 =
        sql_query("SELECT * FROM `delivery_slip` WHERE id_delivery_slip = 1008692")
            .load::<DeliverySlipPlaceholder>(connection)
            .expect("Failed to query delivery_slip table");

    assert_eq!(query_results.len(), 4); // 1 must fail because of csv error
    assert_eq!(
        query_result_sample_1[0],
        DeliverySlipPlaceholder {
            id_delivery_slip: 1011124,
            id_client: 1009681,
            reference: "BL501714".to_string(),
            shipping_date: Some(NaiveDate::from_ymd_opt(2023, 3, 1).unwrap()),
            po_ref: Some("ABC / DEF".to_string()),
            carrier_name: Some("UPS".to_string()),
            status: Some("NA".to_string()),
            tracking_number: Some("250238315641646373".to_string()),
            tracking_link: Some("https://tracking1.com/123".to_string())
        }
    );
    assert_eq!(
        query_result_sample_2[0],
        DeliverySlipPlaceholder {
            id_delivery_slip: 1012515,
            id_client: 1012489,
            reference: "BL501392".to_string(),
            shipping_date: None,
            po_ref: None,
            carrier_name: None,
            status: None,
            tracking_number: None,
            tracking_link: None
        }
    );
    assert_eq!(
        query_result_sample_3[0],
        DeliverySlipPlaceholder {
            id_delivery_slip: 1008692,
            id_client: 1009681,
            reference: "BL501550".to_string(),
            shipping_date: Some(NaiveDate::from_ymd_opt(2023, 3, 1).unwrap()),
            po_ref: Some("ACALOG201138802".to_string()),
            carrier_name: Some("Carrier 1".to_string()),
            status: Some("UN".to_string()),
            tracking_number: Some("TrackingNo3".to_string()),
            tracking_link: None
        }
    );
}

#[derive(QueryableByName, Debug, PartialEq)]
struct DeliverySlipPlaceholder {
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_delivery_slip: u32,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_client: u32,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub reference: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Date>)]
    pub shipping_date: Option<chrono::NaiveDate>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub po_ref: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub carrier_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub status: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub tracking_number: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub tracking_link: Option<String>,
}
