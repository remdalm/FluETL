use chrono::NaiveDate;
use chrono::NaiveTime;
use diesel::{sql_query, QueryableByName, RunQueryDsl};
use std::ops::Range;
use std::process::Command;

use crate::reset_test_database;
use crate::setup_database_connection;

// ****************** //
// test fluetl import order --env-file=.env.test
// ****************** //
#[test]
fn import_order_once() {
    import_order(0..1);
}

#[test]
fn import_order_10_times() {
    import_order(0..10);
}

fn import_order(repeat: Range<i32>) {
    // Arrange
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    // Result
    for _ in repeat {
        let output = Command::new("target/debug/fluetl")
            .args(["import", "order", "--env-file=.env.test"])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Command executed successfully:\n{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Command failed:\n{}", stderr);
        }
    }

    // Assert
    let query_results = sql_query("SELECT * FROM `order`")
        .load::<OrderPlaceholder>(&mut connection)
        .expect("Failed to query order table");

    let query_result_sample_1 = sql_query("SELECT * FROM `order` WHERE id_order = 1113194")
        .load::<OrderPlaceholder>(&mut connection)
        .expect("Failed to query order table");

    let query_result_sample_2 = sql_query("SELECT * FROM `order` WHERE id_order = 1118643")
        .load::<OrderPlaceholder>(&mut connection)
        .expect("Failed to query order table");

    assert_eq!(query_results.len(), 5); //No validation error, all 5 rows are inserted
    assert_eq!(
        query_result_sample_1[0],
        OrderPlaceholder {
            id_order: 1113194,
            id_client: 1009681,
            client_name: Some("Client 1".to_string()),
            order_ref: "OV426946".to_string(),
            date: chrono::NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2023, 6, 7).unwrap(),
                NaiveTime::MIN
            ),
            origin: Some("Web".to_string()),
            order_status: Some("IP".to_string()),
            completion: Some(1), // 0.9% rounded up to 1
            po_ref: Some("P23HA01525".to_string())
        }
    );
    assert_eq!(
        query_result_sample_2[0],
        OrderPlaceholder {
            id_order: 1118643,
            id_client: 1010265,
            client_name: Some("Client 5".to_string()),
            order_ref: "OV427619".to_string(),
            date: chrono::NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2023, 3, 17).unwrap(),
                NaiveTime::MIN
            ),
            origin: None,
            order_status: None,
            completion: Some(100),
            po_ref: Some("WEB73714".to_string())
        }
    );
}

#[derive(QueryableByName, Debug, PartialEq)]
struct OrderPlaceholder {
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_order: u32,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_client: u32,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub client_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub order_ref: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Unsigned<diesel::sql_types::Integer>>)]
    pub completion: Option<u32>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub po_ref: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub origin: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub date: chrono::NaiveDateTime,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub order_status: Option<String>,
}
