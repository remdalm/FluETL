use core::ops::Deref;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type DbConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

// Set up a connection pool for the specified database URL
pub fn establish_connection_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool")
}
