use crate::domain::mapping_client::MappingClient;
use crate::infrastructure::database::models::mapping_client::{
    MappingClientModel, MappingClientSource,
};
use crate::infrastructure::InfrastructureError;

use super::MappingError;

impl TryFrom<MappingClientSource> for MappingClient {
    type Error = MappingError;
    fn try_from(source: MappingClientSource) -> Result<MappingClient, MappingError> {
        let dto_result: Result<MappingClientSourceDTO, InfrastructureError> = source.try_into();
        let dto = dto_result.map_err(MappingError::Infrastructure)?;

        MappingClient::from_i32(dto.id, dto.id_source_client).map_err(|e| e.into())
    }
}

pub struct MappingClientSourceDTO {
    pub id_source_client: i32,
    pub id_source_contact: i32,
    pub id: i32,
}

impl TryFrom<MappingClientSource> for MappingClientSourceDTO {
    type Error = InfrastructureError;
    fn try_from(
        mapping_client_source: MappingClientSource,
    ) -> Result<MappingClientSourceDTO, InfrastructureError> {
        if mapping_client_source.id.is_none() {
            return Err(InfrastructureError::InconsistentDataError(
                "Fetching mapping client source with null id is not allowed".to_owned(),
            ));
        }
        Ok(MappingClientSourceDTO {
            id_source_client: mapping_client_source.id_source_client,
            id_source_contact: mapping_client_source.id_source_contact,
            id: mapping_client_source.id.unwrap(),
        })
    }
}

impl From<MappingClient> for MappingClientModel {
    fn from(entity: MappingClient) -> Self {
        Self {
            id_customer: entity.id_customer(),
            idp_id_client: entity.idp_id_client(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::mapping_client::tests::mapping_client_fixture,
        infrastructure::database::models::mapping_client::tests::mapping_client_source_model_fixture,
        interface_adapters::mappers::GenericMapperParser,
    };

    struct Parser;
    //impl ModelToEntityParser<MappingClientSource, MappingClient> for Parser {}
    impl GenericMapperParser<MappingClientSource, MappingClient> for Parser {}

    use super::*;
    #[test]
    fn test_convert_source_to_entity() {
        let source_fixtures = mapping_client_source_model_fixture();
        let results = Parser.parse_all(source_fixtures.to_vec());

        let mapping_client_fixtures = mapping_client_fixture();

        assert!(results[0].is_ok(), "Expected successful conversion");
        assert_eq!(results[0].as_ref().unwrap(), &mapping_client_fixtures[0]);

        assert!(results[1].is_ok(), "Expected successful conversion");
        assert_eq!(results[1].as_ref().unwrap(), &mapping_client_fixtures[1]);
    }
}
