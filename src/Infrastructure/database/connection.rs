use std::env;

use diesel::prelude::*;
use lazy_static::lazy_static;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type DbConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

lazy_static! {
    static ref CONNECTION_POOL: DbPool = {
        // Load environment variables from .env file
        // TODO: Must be done in main.rs
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in the .env file");

        establish_connection_pool(&database_url)
    };
}

// Set up a connection pool for the specified database URL
pub fn establish_connection_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool")
}

// Function to get a reference to the connection pool
pub fn get_connection_pool() -> &'static DbPool {
    &*CONNECTION_POOL
}

pub fn get_pooled_connection() -> DbConnection {
    get_connection_pool()
        .get()
        .expect("Failed to get connection")
}

#[cfg(test)]
pub(crate) mod tests {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    use super::*;
    lazy_static! {
        // Define a static connection pool
        static ref TEST_CONNECTION_POOL: DbPool = {
            // Load environment variables from .env file
            dotenvy::dotenv().ok();

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

    // Function to set up the test database using the initial migration's up.sql
    pub fn setup_test_database(connection: &mut DbConnection) {
        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Failed to run pending migrations");
    }

    // Function to clean up the test database
    pub fn teardown_test_database(connection: &mut DbConnection) {
        connection
            .revert_all_migrations(MIGRATIONS)
            .expect("Failed to reverse pending migrations");
    }

    pub fn reset_test_database(connection: &mut DbConnection) {
        teardown_test_database(connection);
        setup_test_database(connection);
    }
}
