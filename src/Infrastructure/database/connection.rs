use std::env;

use diesel::prelude::*;
use lazy_static::lazy_static;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type DbConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub(crate) enum Database {
    Target,
    LegacyStaging,
}

lazy_static! {
    static ref TARGET_CONNECTION_POOL: DbPool = {
        // Load environment variables from .env file
        // TODO: Must be done in main.rs
        dotenvy::dotenv().ok();

        let target_database_url = env::var("TARGET_DATABASE_URL")
            .expect("TARGET_DATABASE_URL must be set in the .env file");

        establish_connection_pool(&target_database_url)
    };
}

lazy_static! {
    static ref LEGACT_STAGING_CONNECTION_POOL: DbPool = {
        // Load environment variables from .env file
        // TODO: Must be done in main.rs
        dotenvy::dotenv().ok();

        let legacy_staging_database_url = env::var("LEGACY_STAGING_DATABASE_URL")
            .expect("LEGACY_STAGING_DATABASE_URL must be set in the .env file");

        establish_connection_pool(&legacy_staging_database_url)
    };
}

// Set up a connection pool for the specified database URL
pub fn establish_connection_pool(target_database_url: &str) -> DbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(target_database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool")
}

// Function to get a reference to the connection pool
fn get_target_pooled_connection() -> &'static DbPool {
    &*TARGET_CONNECTION_POOL
}

fn get_legacy_staging_pooled_connection() -> &'static DbPool {
    &*LEGACT_STAGING_CONNECTION_POOL
}

pub(crate) fn get_pooled_connection(db: Database) -> DbConnection {
    let result = match db {
        Database::Target => get_target_pooled_connection(),
        Database::LegacyStaging => get_legacy_staging_pooled_connection(),
    };
    result.get().expect("Failed to get connection")
}

pub(crate) trait HasConnection {
    fn get_pooled_connection() -> DbConnection;
}

pub(crate) struct HasTargetConnection;
impl HasConnection for HasTargetConnection {
    fn get_pooled_connection() -> DbConnection {
        get_pooled_connection(Database::Target)
    }
}

pub(crate) struct HasLegacyStagingConnection;
impl HasConnection for HasLegacyStagingConnection {
    fn get_pooled_connection() -> DbConnection {
        get_pooled_connection(Database::LegacyStaging)
    }
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
            dotenvy::from_path(".env.unit.test").ok();

            // Get the value of TEST_TARGET_DATABASE_URL from the environment
            let target_database_url = env::var("TEST_TARGET_DATABASE_URL")
                .expect("TEST_TARGET_DATABASE_URL must be set in the .env file");

            establish_connection_pool(&target_database_url)
        };
    }

    pub(crate) struct HasTestConnection;
    impl HasConnection for HasTestConnection {
        fn get_pooled_connection() -> DbConnection {
            get_test_pooled_connection()
        }
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
