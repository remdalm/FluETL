use crate::domain::{language::Language, DomainError};

use super::{locale::Locale, Translation};

#[derive(Debug, Clone, PartialEq)]
pub struct LocalizedItem {
    language: Language,
    name: Translation,
}

impl LocalizedItem {
    pub fn new(language: Language, name: Translation) -> Self {
        LocalizedItem { language, name }
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn name(&self) -> &Translation {
        &self.name
    }
}

pub trait LocalizedItemFactory {
    fn make_from_language(&self, language: &Language) -> Result<LocalizedItem, DomainError>;
    fn get_language_locale(&self) -> &Locale;
    fn get_entity_id(&self) -> u32;
}

pub mod tests {
    use crate::domain::language::tests::language_fixtures;

    use super::*;
    pub fn localized_item_fixtures() -> [LocalizedItem; 3] {
        [
            LocalizedItem::new(
                language_fixtures()[0].clone(),
                Translation::new("Bottle".to_string()).unwrap(),
            ),
            LocalizedItem::new(
                language_fixtures()[1].clone(),
                Translation::new("Bouteille".to_string()).unwrap(),
            ),
            LocalizedItem::new(
                language_fixtures()[0].clone(),
                Translation::new("Plate".to_string()).unwrap(),
            ),
        ]
    }
}
