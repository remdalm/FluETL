extern crate diesel;
extern crate fluetl;

use diesel::{sql_query, RunQueryDsl};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use fluetl::infrastructure::database::connection::{establish_connection_pool, DbConnection};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

mod cli_sub_routines;

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
