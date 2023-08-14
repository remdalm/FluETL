use std::env;

use criterion::{criterion_group, criterion_main, Criterion};
use idempiere_data_extractor::{
    benches::database_connection::{establish_connection_pool, DbConnection},
    benches::SingleRowInsertable,
    fixtures::{mapping_client_model_fixture, order_model_fixtures},
};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// Define benchmark functions
pub fn benchmark_upsert_recommended(c: &mut Criterion) {
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    let mapping_client = mapping_client_model_fixture();
    mapping_client
        .insert(&mut connection)
        .expect("Failed to insert mapping client");
    let order = &order_model_fixtures()[0];
    order
        .insert(&mut connection)
        .expect("Failed to insert order");

    c.bench_function("recommended_upsert", |b| {
        b.iter(|| {
            order.upsert(&mut connection).unwrap();
        })
    });
}

pub fn benchmark_upsert(c: &mut Criterion) {
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    let mapping_client = mapping_client_model_fixture();
    mapping_client
        .insert(&mut connection)
        .expect("Failed to insert mapping client");
    let order = &order_model_fixtures()[0];
    order
        .insert(&mut connection)
        .expect("Failed to insert order");

    c.bench_function("homemade_upsert", |b| {
        b.iter(|| {
            order.homemade_upsert(&mut connection).unwrap();
        })
    });
}

fn setup_database_connection() -> DbConnection {
    dotenvy::dotenv().ok();
    println!("benchmark_upsert_recommended");
    let database_url =
        env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set in the .env file");

    establish_connection_pool(&database_url).get().unwrap()
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

criterion_group!(
    upsert_benchmark,
    benchmark_upsert,
    benchmark_upsert_recommended
);

// Run the benchmarks
criterion_main!(upsert_benchmark);
