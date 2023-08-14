mod application;
mod domain;
mod infrastructure;

#[cfg(test)]
pub mod tests {
    pub mod common;
}

pub mod fixtures;

// Export code for benchmarking
// Probably not the best way to do this
pub mod benches {
    pub use super::infrastructure::database::connection as database_connection;
    pub use super::infrastructure::database::models::{
        order::OrderModel, Model, SingleRowInsertable, SingleRowUpdatable,
    };
}
