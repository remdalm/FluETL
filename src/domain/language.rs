use super::{vo::locale::Locale, DomainEntity, DomainError};

#[derive(Debug, PartialEq, Clone)]
pub struct Language {
    language_id: u32,
    locale: Locale,
}

impl Language {
    pub fn language_id(&self) -> u32 {
        self.language_id
    }

    pub fn locale(&self) -> &Locale {
        &self.locale
    }
}

impl DomainEntity for Language {}

pub struct LanguageDomainFactory {
    pub language_id: u32,
    pub locale: String,
}

impl LanguageDomainFactory {
    pub fn make(self) -> Result<Language, DomainError> {
        Ok(Language {
            language_id: self.language_id,
            locale: Locale::try_from(self.locale.as_str())?,
        })
    }
}

pub mod tests {
    use super::*;
    pub fn language_fixtures() -> [Language; 2] {
        [
            Language {
                language_id: 1,
                locale: Locale::try_from("en_US").unwrap(),
            },
            Language {
                language_id: 2,
                locale: Locale::try_from("fr_FR").unwrap(),
            },
        ]
    }
}
