use crate::{
    domain::MappingClient,
    infrastructure::database::models::mapping_client::{MappingClientModel, MappingClientSource},
};

use super::*;

pub struct ImportMappingClientUseCase;
impl HasLegacyStagingConnection for ImportMappingClientUseCase {}
impl HasTargetConnection for ImportMappingClientUseCase {}

impl CanReadAllModelUseCase for ImportMappingClientUseCase {
    type ModelImpl = MappingClientSource;
    // fn concrete_model(&self) -> Self::ModelImpl {
    //     MappingClientSource::default()
    // }
}

impl CanPersistIntoDatabaseUseCase<MappingClient, MappingClientModel>
    for ImportMappingClientUseCase
{
}

impl ModelToEntityParser<MappingClientSource, MappingClient> for ImportMappingClientUseCase {}

impl ImportModelUseCase<MappingClientSource, MappingClient, MappingClientModel>
    for ImportMappingClientUseCase
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        infrastructure::database::connection::tests::{
            get_test_pooled_connection, reset_test_database,
        },
        infrastructure::database::connection::DbConnection,
        infrastructure::database::models::mapping_client::tests::{
            insert_batch_to_mapping_client_source_db, read_mapping_client,
        },
    };

    struct ImportMappingClientUseCaseTest;

    impl HasTargetConnection for ImportMappingClientUseCaseTest {
        fn get_pooled_connection(&self) -> DbConnection {
            get_test_pooled_connection()
        }
    }

    impl HasLegacyStagingConnection for ImportMappingClientUseCaseTest {
        fn get_pooled_connection(&self) -> DbConnection {
            get_test_pooled_connection()
        }
    }

    impl CanReadAllModelUseCase for ImportMappingClientUseCaseTest {
        type ModelImpl = MappingClientSource;
    }

    impl CanPersistIntoDatabaseUseCase<MappingClient, MappingClientModel>
        for ImportMappingClientUseCaseTest
    {
    }

    impl ModelToEntityParser<MappingClientSource, MappingClient> for ImportMappingClientUseCaseTest {}

    impl ImportModelUseCase<MappingClientSource, MappingClient, MappingClientModel>
        for ImportMappingClientUseCaseTest
    {
    }

    #[test]
    fn test_order_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);
        insert_batch_to_mapping_client_source_db(&mut connection).expect("Failed to insert data");

        // Result
        let use_case = ImportMappingClientUseCaseTest;
        let errors = use_case.execute();

        // Assert
        assert!(errors.is_none(), "Failed to execute use case");
        let persisted_mapping_client = read_mapping_client(&mut connection);
        assert_eq!(persisted_mapping_client.len(), 2);
        assert_eq!(persisted_mapping_client[0].id_customer, 1);
        assert_eq!(persisted_mapping_client[1].id_customer, 2);
    }

    // TODO: Test with failure
}
