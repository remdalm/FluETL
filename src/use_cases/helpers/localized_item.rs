use std::collections::HashMap;
use std::fmt::Debug;

use log::{debug, error};

use crate::{
    domain::{
        language::Language,
        vo::localized_item::{LocalizedItem, LocalizedItemFactory},
        DomainError,
    },
    interface_adapters::mappers::MappingError,
    use_cases::UseCaseError,
};

use super::language::CanFetchLanguages;

pub(crate) trait ImportLocalizedItem<T, S>: CanFetchLanguages
where
    T: TryFrom<S, Error = MappingError> + LocalizedItemFactory + Debug,
{
    fn parse(&self) -> Result<Vec<T>, UseCaseError> {
        let source = self.source()?;
        let mut item_name_factories: Vec<T> = Vec::new();
        source.into_iter().for_each(|dto| {
            let factory: Result<T, MappingError> = dto.try_into();
            if let Ok(factory) = factory {
                item_name_factories.push(factory);
            } else {
                error!(
                    "Failed to parse LocalizedItemFactory: {:?}",
                    factory.unwrap_err()
                );
            }
        });

        Ok(item_name_factories)
    }

    fn make_localized_items(&self) -> Result<Vec<(u32, LocalizedItem)>, Vec<UseCaseError>> {
        debug!("Fetching languages...");
        let languages = Self::fetch_languages().map_err(|e| Vec::from([e]))?;
        debug!("Fetched {} languages", languages.len());
        debug!("Importing translations...");
        let item_name_factories = self.parse().map_err(|e| Vec::from([e]))?;
        debug!("Parsing translations...");
        let item_name_results: Vec<Result<(u32, LocalizedItem), UseCaseError>> =
            item_name_factories
                .into_iter()
                .map(|factory| {
                    let language: &Language = languages
                        .iter()
                        .find(|l| l.locale() == factory.get_language_locale())
                        .ok_or(UseCaseError::Domain(DomainError::ValidationError(format!(
                            "No language match the source language {}",
                            factory.get_language_locale().as_str()
                        ))))?;
                    let item_name = factory
                        .make_from_language(language)
                        .map_err(UseCaseError::Domain)?;
                    Ok((factory.get_entity_id(), item_name))
                })
                .collect();

        debug!("Parsed {} translations", item_name_results.len());
        debug!("Filtering out errors...");

        let mut item_names: Vec<(u32, LocalizedItem)> = Vec::new();
        let mut errors: Vec<UseCaseError> = Vec::new();

        item_name_results
            .into_iter()
            .for_each(|result| match result {
                Ok(item_name) => item_names.push(item_name),
                Err(error) => errors.push(error),
            });

        debug!("Filtered {} errors", errors.len());
        debug!("Total of valid translations: {}", item_names.len());

        if item_names.is_empty() {
            return Err(errors);
        }

        Ok(item_names)
    }

    fn group_localized_items(
        item_names: Vec<(u32, LocalizedItem)>,
    ) -> HashMap<u32, Vec<LocalizedItem>> {
        let mut item_names_hashmap: HashMap<u32, Vec<LocalizedItem>> = HashMap::new();
        item_names.into_iter().for_each(|(entity_id, item_name)| {
            if let Some(item_names_vec) = item_names_hashmap.get_mut(&entity_id) {
                item_names_vec.push(item_name)
            } else {
                item_names_hashmap.insert(entity_id, vec![item_name]);
            }
        });

        item_names_hashmap
    }

    fn source(&self) -> Result<Vec<S>, UseCaseError>;
}
