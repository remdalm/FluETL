use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;

use crate::infrastructure::database::connection::{establish_connection_pool, DbPool};

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
pub fn get_test_connection_pool() -> &'static DbPool {
    &*TEST_CONNECTION_POOL
}
