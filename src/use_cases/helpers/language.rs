use crate::{
    domain::language::Language,
    infrastructure::{
        database::{
            connection::{HasConnection, HasLegacyStagingConnection},
            models::{language::LanguageModel, CanSelectAllModel},
        },
        InfrastructureError,
    },
    use_cases::UseCaseError,
};

pub(crate) trait CanFetchLanguages {
    fn fetch_languages() -> Result<Vec<Language>, UseCaseError> {
        LanguageModel::select_all(&mut HasLegacyStagingConnection::get_pooled_connection())
            .map_err(|e| UseCaseError::Infrastructure(InfrastructureError::DatabaseError(e)))
            .and_then(|models| {
                models
                    .into_iter()
                    .map(|m| m.try_into().map_err(UseCaseError::Mapping))
                    .collect()
            })
    }
}
