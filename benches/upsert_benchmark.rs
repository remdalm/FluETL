use std::env;

use criterion::{criterion_group, criterion_main, Criterion};
use diesel::result::DatabaseErrorKind;
use fluetl::{
    fixtures::{mapping_client_model_fixture, order_model_fixtures},
    infrastructure::database::connection::{establish_connection_pool, DbConnection},
    infrastructure::database::models::{
        CanUpsertModel, OrderModel, SingleRowInsertable, SingleRowUpdatable,
    },
};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// Define benchmark functions
pub fn benchmark_upsert_recommended(c: &mut Criterion) {
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    let mapping_client = &mapping_client_model_fixture()[0];
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

    let mapping_client = &mapping_client_model_fixture()[0];
    mapping_client
        .insert(&mut connection)
        .expect("Failed to insert mapping client");
    let order = &order_model_fixtures()[0];
    order
        .insert(&mut connection)
        .expect("Failed to insert order");

    c.bench_function("homemade_upsert", |b| {
        b.iter(|| {
            homemade_upsert(&order, &mut connection).unwrap();
        })
    });
}

// Just to be convinced that the recommended way is far more optimised and should be used throughout the codebase
pub fn homemade_upsert(
    order_model: &OrderModel,
    connection: &mut DbConnection,
) -> Result<(), diesel::result::Error> {
    let insert_result = order_model.insert(connection);
    if let Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) =
        insert_result
    {
        order_model.update(connection)
    } else {
        insert_result
    }
}

fn setup_database_connection() -> DbConnection {
    dotenvy::dotenv().ok();
    let test_target_database_url = env::var("TEST_TARGET_DATABASE_URL")
        .expect("TEST_TARGET_DATABASE_URL must be set in the .env file");

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

criterion_group!(
    upsert_benchmark,
    benchmark_upsert,
    benchmark_upsert_recommended
);

// Run the benchmarks
criterion_main!(upsert_benchmark);
