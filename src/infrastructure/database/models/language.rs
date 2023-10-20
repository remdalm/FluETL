use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::schema;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use super::{CanSelectAllModel, Model};

#[derive(Queryable, Identifiable, Selectable, Default, Clone, Debug, PartialEq)]
#[diesel(table_name = schema::legacy_staging::language_list)]
#[diesel(primary_key(locale))]
pub struct LanguageModel {
    pub locale: String,
    pub id: i32,
}

impl Model for LanguageModel {}
impl CanSelectAllModel for LanguageModel {
    fn select_all(connection: &mut DbConnection) -> Result<Vec<Self>, DieselError> {
        use self::schema::legacy_staging::language_list::dsl::*;
        language_list
            .filter(id.is_not_null())
            .select(LanguageModel::as_select())
            .load(connection)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::infrastructure::database::connection::tests::{
        get_test_pooled_connection, reset_test_database,
    };

    pub fn language_model_fixture() -> [LanguageModel; 2] {
        [
            LanguageModel {
                locale: "en_US".to_string(),
                id: 1,
            },
            LanguageModel {
                locale: "fr_FR".to_string(),
                id: 2,
            },
        ]
    }

    pub fn insert_languages(connection: &mut DbConnection) -> Result<(), DieselError> {
        use self::schema::legacy_staging::language_list::dsl::*;
        let data = &vec![
            (locale.eq("en_US"), id.eq(1)),
            (locale.eq("fr_FR"), id.eq(2)),
        ];

        diesel::insert_into(language_list)
            .values(data)
            .execute(connection)
            .map(|_| ())
    }

    #[test]
    fn test_read_source() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        insert_languages(&mut connection)
            .expect("Error inserting batch to mapping client source db");

        let result = LanguageModel::select_all(&mut connection).expect("Error reading source");

        let models = language_model_fixture();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], models[0]);
        assert_eq!(result[1], models[1]);
    }
}
