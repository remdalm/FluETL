extern crate diesel;
extern crate fluetl;

use chrono::NaiveDate;
use chrono::NaiveTime;
use diesel::{sql_query, QueryableByName, RunQueryDsl};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Range;
use std::path::Path;
use std::process::Command;

use fluetl::infrastructure::database::connection::{establish_connection_pool, DbConnection};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// ****************** //
// test fluetl --env-file=.env.test import mapping-client
// ******************

#[test]
fn import_mapping_client_once() {
    import_mapping_client(0..1);
}

#[test]
fn import_mapping_client_10_times() {
    import_mapping_client(0..10);
}

fn import_mapping_client(repeat: Range<i32>) {
    // Arrange
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    insert_raw_sql("tests/fixtures/mapping_client_source.sql", &mut connection)
        .expect("Failed to insert mapping_client_source.sql fixture file");

    // Result
    for _ in repeat {
        let output = Command::new("target/debug/fluetl")
            .args(&["--env-file=.env.test", "import", "mapping-client"])
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

    //Assert
    let query_results = sql_query("SELECT * From `mapping_client_contact`")
        .load::<MappingClientPlaceholder>(&mut connection)
        .expect("Failed to query mapping_client_contact table");

    let query_result_sample_1 =
        sql_query("SELECT * From `mapping_client_contact` WHERE idp_id_client = 1009681")
            .load::<MappingClientPlaceholder>(&mut connection)
            .expect("Failed to query mapping_client_contact table");

    let query_result_sample_2 =
        sql_query("SELECT * From `mapping_client_contact` WHERE idp_id_client = 1010265")
            .load::<MappingClientPlaceholder>(&mut connection)
            .expect("Failed to query mapping_client_contact table");

    assert_eq!(query_results.len(), 4); // mapping_client_source.sql has 5 rows but one is ignored because of null id_customer
    assert_eq!(
        query_result_sample_1[0],
        MappingClientPlaceholder {
            id_customer: 1,
            idp_id_client: 1009681
        }
    );
    assert_eq!(
        query_result_sample_2[0],
        MappingClientPlaceholder {
            id_customer: 4,
            idp_id_client: 1010265
        }
    );
}

#[derive(QueryableByName, Debug, PartialEq)]
struct MappingClientPlaceholder {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id_customer: i32,
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub idp_id_client: i32,
}

// ****************** //
// test fluetl --env-file=.env.test import order
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
            .args(&["--env-file=.env.test", "import", "order"])
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
            order_status: Some("Commande en attente de confirmation".to_string()),
            delivery_status: Some("En attente".to_string()),
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
            delivery_status: Some("Livré en intégralité".to_string()),
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
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub delivery_status: Option<String>,
}

// ****************** //
// test fluetl --env-file=.env.test import orderline
// ****************** //
#[test]
fn import_order_line_once() {
    import_order_line(0..1);
}

#[test]
fn import_order_line_10_times() {
    import_order_line(0..10);
}

fn import_order_line(repeat: Range<i32>) {
    // Arrange
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    // Import Orders:
    Command::new("target/debug/fluetl")
        .args(&["--env-file=.env.test", "import", "order"])
        .output()
        .expect("Failed to execute command");

    // Result
    for _ in repeat {
        let output = Command::new("target/debug/fluetl")
            .args(&["--env-file=.env.test", "import", "orderline"])
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
    let query_results = sql_query("SELECT * FROM `order_line`")
        .load::<OrderLinePlaceholder>(&mut connection)
        .expect("Failed to query order_line table");

    let query_result_sample_1 =
        sql_query("SELECT * FROM `order_line` WHERE id_order_line = 1833200")
            .load::<OrderLinePlaceholder>(&mut connection)
            .expect("Failed to query order_line table");

    let query_result_sample_2 =
        sql_query("SELECT * FROM `order_line` WHERE id_order_line = 1833202")
            .load::<OrderLinePlaceholder>(&mut connection)
            .expect("Failed to query order_line table");

    assert_eq!(query_results.len(), 3); // 2 must fail because of validation error
    assert_eq!(
        query_result_sample_1[0],
        OrderLinePlaceholder {
            id_order_line: 1833200,
            id_order: 1113194,
            product_ref: "9995269".to_string(),
            product_name: Some("Hélice + Vis ".to_string()),
            qty_ordered: 10,
            qty_reserved: 5,
            qty_delivered: 5,
            due_date: Some(NaiveDate::from_ymd_opt(2023, 6, 7).unwrap())
        }
    );
    assert_eq!(
        query_result_sample_2[0],
        OrderLinePlaceholder {
            id_order_line: 1833202,
            id_order: 1112737,
            product_ref: "CDE20".to_string(),
            product_name: None,
            qty_ordered: 30,
            qty_reserved: 15,
            qty_delivered: 15,
            due_date: None
        }
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
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>)]
    pub product_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub qty_ordered: u32,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub qty_reserved: u32,
    #[diesel(sql_type = diesel::sql_types::Unsigned<diesel::sql_types::Integer>)]
    pub qty_delivered: u32,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Date>)]
    pub due_date: Option<chrono::NaiveDate>,
}

// ****************** //
// Helper functions
// ****************** //
fn insert_raw_sql<P>(
    path: P,
    connection: &mut DbConnection,
) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            // Execute each non-empty line as a query
            sql_query(line).execute(connection)?;
        }
    }

    println!("SQL file executed successfully");
    Ok(())
}

fn setup_database_connection() -> DbConnection {
    dotenvy::from_path(".env.test").ok();
    let test_target_database_url =
        env::var("TARGET_DATABASE_URL").expect("TARGET_DATABASE_URL must be set in the .env file");

    establish_connection_pool(&test_target_database_url)
        .get()
        .unwrap()
}

// Function to set up the test database using the initial migration's up.sql
fn setup_test_database(connection: &mut DbConnection) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run pending migrations");
}

// Function to clean up the test database
fn teardown_test_database(connection: &mut DbConnection) {
    connection
        .revert_all_migrations(MIGRATIONS)
        .expect("Failed to reverse pending migrations");
}

fn reset_test_database(connection: &mut DbConnection) {
    teardown_test_database(connection);
    setup_test_database(connection);
}
