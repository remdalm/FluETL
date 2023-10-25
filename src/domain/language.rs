use super::{vo::locale::Locale, DomainEntity, DomainError};

#[derive(Debug, Clone)]
pub struct Language {
    id: u32,
    locale: Locale,
}

impl Language {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn locale(&self) -> &Locale {
        &self.locale
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl DomainEntity for Language {}

pub struct LanguageDomainFactory {
    pub id: u32,
    pub locale: String,
}

impl LanguageDomainFactory {
    pub fn make(self) -> Result<Language, DomainError> {
        Ok(Language {
            id: self.id,
            locale: Locale::try_from(self.locale.as_str())?,
        })
    }
}

pub mod tests {
    use super::*;
    pub fn language_fixtures() -> [Language; 2] {
        [
            Language {
                id: 1,
                locale: Locale::try_from("en_US").unwrap(),
            },
            Language {
                id: 2,
                locale: Locale::try_from("fr_FR").unwrap(),
            },
        ]
    }

    #[test]
    fn test_language_eq() {
        let language1 = language_fixtures()[0].clone();
        let language2 = language_fixtures()[1].clone();
        let language3 = Language {
            id: 1,
            locale: Locale::try_from("fr_FR").unwrap(),
        };
        assert_ne!(language1, language2);
        assert_eq!(language1, language3);
    }
}
