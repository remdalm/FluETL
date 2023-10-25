use diesel::{sql_query, QueryableByName, RunQueryDsl};
use serial_test::serial;
use std::ops::Range;
use std::process::Command;

use crate::reset_test_database;
use crate::setup_database_connection;
use crate::{insert_raw_sql, panic_if_stdout_contains_error};

// ****************** //
// test fluetl --env-file=.env.test import mapping-client
// ******************

#[test]
#[serial]
fn import_mapping_client_once() {
    import_mapping_client(0..1);
}

#[test]
#[serial]
fn import_mapping_client_10_times() {
    import_mapping_client(0..10);
}

fn import_mapping_client(repeat: Range<i32>) {
    // Arrange
    let mut connection = setup_database_connection();
    reset_test_database(&mut connection);

    insert_raw_sql("tests/fixtures/mapping_client_source.sql", &mut connection)
        .expect("Failed to insert mapping_client_source.sql fixture file");

    // Result
    for _ in repeat {
        let output = Command::new("target/debug/fluetl")
            .args(["import", "mapping-client", "--env-file=.env.test"])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            panic_if_stdout_contains_error(&stdout);
            println!("Command executed successfully:\n{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Command failed:\n{}", stderr);
        }
    }

    //Assert
    let query_results = sql_query("SELECT * From `mapping_client_contact`")
        .load::<MappingClientPlaceholder>(&mut connection)
        .expect("Failed to query mapping_client_contact table");

    let query_result_sample_1 =
        sql_query("SELECT * From `mapping_client_contact` WHERE idp_id_client = 1009681")
            .load::<MappingClientPlaceholder>(&mut connection)
            .expect("Failed to query mapping_client_contact table");

    let query_result_sample_2 =
        sql_query("SELECT * From `mapping_client_contact` WHERE idp_id_client = 1010265")
            .load::<MappingClientPlaceholder>(&mut connection)
            .expect("Failed to query mapping_client_contact table");

    assert_eq!(query_results.len(), 4); // mapping_client_source.sql has 5 rows but one is ignored because of null id_customer
    assert_eq!(
        query_result_sample_1[0],
        MappingClientPlaceholder {
            id_customer: 1,
            idp_id_client: 1009681
        }
    );
    assert_eq!(
        query_result_sample_2[0],
        MappingClientPlaceholder {
            id_customer: 4,
            idp_id_client: 1010265
        }
    );
}

#[derive(QueryableByName, Debug, PartialEq)]
struct MappingClientPlaceholder {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id_customer: i32,
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub idp_id_client: i32,
}
