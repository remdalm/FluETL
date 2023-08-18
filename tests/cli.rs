extern crate diesel;
extern crate fluetl;

use diesel::{sql_query, QueryableByName, RunQueryDsl};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Range;
use std::path::Path;
use std::process::Command;

use fluetl::benches::database_connection::{establish_connection_pool, DbConnection};

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
    let query_results = sql_query("SELECT * From mapping_client_contact")
        .load::<MappingClientPlaceholder>(&mut connection)
        .expect("Failed to query mapping_client_contact table");

    let query_result_sample_1 =
        sql_query("SELECT * From mapping_client_contact WHERE idp_id_client = 1009681")
            .load::<MappingClientPlaceholder>(&mut connection)
            .expect("Failed to query mapping_client_contact table");

    let query_result_sample_2 =
        sql_query("SELECT * From mapping_client_contact WHERE idp_id_client = 1010265")
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
    #[sql_type = "diesel::sql_types::Integer"]
    pub id_customer: i32,
    #[sql_type = "diesel::sql_types::Integer"]
    pub idp_id_client: i32,
}

// ****************** //
// test fluetl --env-file=.env.test import order
// ****************** //
#[test]
fn import_order_once() {
    // Arrange
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    // Result
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

    // Assert
    let query_results = sql_query("SELECT * From order")
        .load::<OrderPlaceholder>(&mut connection)
        .expect("Failed to query order table");
}

#[derive(QueryableByName, Debug, PartialEq)]
struct OrderPlaceholder {
    #[sql_type = "diesel::sql_types::Integer"]
    pub id_order: i32,
    #[sql_type = "diesel::sql_types::Integer"]
    pub id_client: i32,
    #[sql_type = "diesel::sql_types::Varchar"]
    pub order_ref: String,
    #[sql_type = "diesel::sql_types::Timestamp"]
    pub date: chrono::NaiveDateTime,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::VarChar>"]
    pub order_status: Option<String>,
    #[sql_type = "diesel::sql_types::Nullable<diesel::sql_types::VarChar>"]
    pub delivery_status: Option<String>,
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