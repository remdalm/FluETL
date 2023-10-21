use crate::domain::language::Language;

use super::Translation;

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
