mod application;
mod domain;
mod infrastructure;

pub mod fixtures;

// Export code for benchmarking
// TODO: Probably not the best way to do this
pub mod benches {
    pub use super::infrastructure::database::connection as database_connection;
    pub use super::infrastructure::database::models::{
        order::OrderModel, Model, SingleRowInsertable, SingleRowUpdatable,
    };
}
