use crate::domain::language::{Language, LanguageDomainFactory};
use crate::infrastructure::database::models::language::LanguageModel;
use crate::infrastructure::database::models::mapping_client::MappingClientSource;
use crate::infrastructure::InfrastructureError;

use super::MappingError;

impl TryFrom<LanguageModel> for Language {
    type Error = MappingError;
    fn try_from(source: LanguageModel) -> Result<Language, MappingError> {
        if source.id < 1 {
            return Err(MappingError::Infrastructure(
                InfrastructureError::InconsistentDataError("Id must be greater than 0".to_string()),
            ));
        }
        LanguageDomainFactory {
            id: source.id as u32,
            locale: source.locale,
        }
        .make()
        .map_err(|e| e.into())
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
