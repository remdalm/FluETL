mod domain;
pub mod infrastructure;
pub mod interface_adapters;
mod use_cases;

pub mod benchmark_fixtures {
    pub use super::infrastructure::database::models::{
        mapping_client::bench::mapping_client_model_fixture, order::bench::order_model_fixtures,
    };
}

#[cfg(test)]
pub mod tests {
    const UNIT_TEST_ENV_PATH: &str = ".env.unit.test";
    pub fn load_unit_test_env() {
        dotenvy::from_path(UNIT_TEST_ENV_PATH).expect("Failed to load unit test env file");
    }
}
