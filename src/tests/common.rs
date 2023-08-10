use diesel::MysqlConnection;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::{env, error::Error, path::PathBuf};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

use crate::infrastructure::database::connection::{
    establish_connection_pool, DbConnection, DbPool,
};

lazy_static! {
    // Define a static connection pool
    static ref TEST_CONNECTION_POOL: DbPool = {
        // Load environment variables from .env file
        dotenv().ok();

        // Get the value of TEST_DATABASE_URL from the environment
        let database_url = env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set in the .env file");

        establish_connection_pool(&database_url)
    };
}

// Function to get a reference to the connection pool
fn get_test_connection_pool() -> &'static DbPool {
    &*TEST_CONNECTION_POOL
}

pub fn get_test_pooled_connection() -> DbConnection {
    get_test_connection_pool()
        .get()
        .expect("Failed to get connection")
}

// fn execute_sql_file(connection: &mut DbConnection, file_path: PathBuf) {
//     let sql_content = std::fs::read_to_string(file_path).expect("Failed to read SQL file");
//     connection
//         .batch_execute(&sql_content)
//         .expect("Failed to execute SQL");
// }

// Function to set up the test database using the initial migration's up.sql
pub fn setup_test_database(connection: &mut DbConnection) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run pending migrations");
}

// Function to clean up the test database
pub fn teardown_test_database(connection: &mut DbConnection) {
    // // TODO: Run migration instead of executing SQL file
    // let cleanup_sql = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //     .join("migrations")
    //     .join("2020-01-01-000000_initial")
    //     .join("down.sql");
    // execute_sql_file(connection, cleanup_sql);
}
