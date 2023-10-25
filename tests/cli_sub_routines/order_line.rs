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
// test fluetl import orderline --env-file=.env.test
// ****************** //
#[test]
#[serial]
fn import_order_line_once() {
    import_order_line(0..1);
}

#[test]
#[serial]
fn import_order_line_10_times() {
    import_order_line(0..10);
}

fn import_order_line(repeat: Range<i32>) {
    // Arrange
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    insert_raw_sql("tests/fixtures/languages.sql", &mut connection)
        .expect("Failed to insert languages.sql fixture file");

    // Import Orders:
    Command::new("target/debug/fluetl")
        .args(["import", "order", "--env-file=.env.test"])
        .output()
        .expect("Failed to execute command");

    // Result
    for _ in repeat {
        let output = Command::new("target/debug/fluetl")
            .args(["import", "orderline", "--env-file=.env.test"])
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

    assert_order_line_results(&mut connection);
}

// ****************** //
// test fluetl import orderline -b -s2 --env-file=.env.test
// ****************** //
#[test]
#[serial]
fn import_order_line_batch() {
    // Arrange
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    insert_raw_sql("tests/fixtures/languages.sql", &mut connection)
        .expect("Failed to insert languages.sql fixture file");

    // Import Orders:
    Command::new("target/debug/fluetl")
        .args(["import", "order", "--env-file=.env.test"])
        .output()
        .expect("Failed to execute command");

    let output = Command::new("target/debug/fluetl")
        .args(["import", "orderline", "-b", "-s2", "--env-file=.env.test"])
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
    assert_order_line_results(&mut connection);
}

fn assert_order_line_results(connection: &mut DbConnection) {
    // Assert Order Line Table
    let order_line_query_results = sql_query("SELECT * FROM `order_line`")
        .load::<OrderLinePlaceholder>(connection)
        .expect("Failed to query order_line table");

    let order_line_query_result_sample_1 =
        sql_query("SELECT * FROM `order_line` WHERE id_order_line = 1833200")
            .load::<OrderLinePlaceholder>(connection)
            .expect("Failed to query order_line table");

    let order_line_query_result_sample_2 =
        sql_query("SELECT * FROM `order_line` WHERE id_order_line = 1833202")
            .load::<OrderLinePlaceholder>(connection)
            .expect("Failed to query order_line table");

    assert_eq!(order_line_query_results.len(), 3); // 2 must fail because of validation error
    assert_eq!(
        order_line_query_result_sample_1[0],
        OrderLinePlaceholder {
            id_order_line: 1833200,
            id_order: 1113194,
            product_ref: "9995269".to_string(),
            qty_ordered: 10,
            qty_reserved: 5,
            qty_delivered: 5,
            due_date: Some(NaiveDate::from_ymd_opt(2023, 6, 7).unwrap())
        }
    );
    assert_eq!(
        order_line_query_result_sample_2[0],
        OrderLinePlaceholder {
            id_order_line: 1833202,
            id_order: 1112737,
            product_ref: "CDE20".to_string(),
            qty_ordered: 30,
            qty_reserved: 15,
            qty_delivered: 15,
            due_date: None
        }
    );

    // Assert Order Line Lang Table
    let order_line_item_query_results = sql_query("SELECT * FROM `order_line_lang`")
        .load::<OrderLineItemPlaceholder>(connection)
        .expect("Failed to query order_line_lang table");

    let order_line_item_query_result_sample_1 =
        sql_query("SELECT * FROM `order_line_lang` WHERE id_order_line = 1833200")
            .load::<OrderLineItemPlaceholder>(connection)
            .expect("Failed to query order_line table");

    let order_line_item_query_result_sample_2 =
        sql_query("SELECT * FROM `order_line_lang` WHERE id_order_line = 1833202")
            .load::<OrderLineItemPlaceholder>(connection)
            .expect("Failed to query order_line table");

    assert_eq!(order_line_item_query_results.len(), 4); // 1 with es_CO locale and 1 link to an imcomplete order must fail
    assert_eq!(
        order_line_item_query_result_sample_1,
        vec![
            OrderLineItemPlaceholder {
                id_order_line: 1833200,
                id_lang: 1,
                product_name:
                    "EN Panier de skimmer GM Standard Liner/Béton + anse - Nouveau Design (ASTRAL)"
                        .to_string()
            },
            OrderLineItemPlaceholder {
                id_order_line: 1833200,
                id_lang: 2,
                product_name:
                    "FR Panier de skimmer GM Standard Liner/Béton + anse - Nouveau Design (ASTRAL)"
                        .to_string()
            }
        ]
    );
    assert_eq!(
        order_line_item_query_result_sample_2,
        vec![
            OrderLineItemPlaceholder {
                id_order_line: 1833202,
                id_lang: 1,
                product_name:
                    "En OBSOLETE - Volet SK Pisc. Hors-Sol - 105D063 - Remplacé par 4411030601 (SNTE)"
                        .to_string()
            },
            OrderLineItemPlaceholder {
                id_order_line: 1833202,
                id_lang: 2,
                product_name:
                    "FR OBSOLETE - Volet SK Pisc. Hors-Sol - 105D063 - Remplacé par 4411030601 (SNTE)"
                        .to_string()
            }
        ]
    );
}

#[derive(QueryableByName, Debug, PartialEq)]
struct OrderLinePlaceholder {
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_order_line: u32,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_order: u32,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub product_ref: String,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub qty_ordered: u32,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub qty_reserved: u32,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub qty_delivered: u32,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Date>)]
    pub due_date: Option<chrono::NaiveDate>,
}

#[derive(QueryableByName, Debug, PartialEq)]
struct OrderLineItemPlaceholder {
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_order_line: u32,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub id_lang: u32,
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub product_name: String,
}
