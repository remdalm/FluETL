use diesel::{prelude::*, result::Error as DieselError};

use crate::infrastructure::database::{
    connection::{DbConnection, HasConnection, HasLegacyStagingConnection},
    schema,
};

use super::{CanSelectAllModel, Model};

#[derive(Queryable, Identifiable, Selectable, Default, Clone, Debug, PartialEq)]
#[diesel(table_name = schema::legacy_staging::staging_product)]
#[diesel(primary_key(id_source))]
pub struct ProductLegacyStagingModel {
    pub id_source: i32,
    pub id: Option<i32>,
    // pub id_source_manufacturer: Option<i32>,
    // pub isbom: bool,
    // pub id_tax_rule: i32,
    // pub name_fr: String,
    // pub reference: String,
    // pub reference_irrijardin: Option<String>,
    // pub price: Decimal,
    // pub active: bool,
    // pub description_fr: Option<String>,
    // pub weight: Option<Decimal>,
    // pub discontinued: bool,
    // pub diametre_ext: i32,
    // pub diametre_int: i32,
    // pub entraxe_2_fixations: i32,
    // pub entraxe_diam: i32,
    // pub entraxe_largeur: i32,
    // pub entraxe_longueur: i32,
    // pub epaisseur: i32,
    // pub hauteur: i32,
    // pub largeur_ext: i32,
    // pub largeur_int: i32,
    // pub longueur_ext: i32,
    // pub longueur_int: i32,
    // pub diametre_int_bas: i32,
    // pub diametre_int_haut: i32,
    // pub replenishment_time: Option<i32>,
    // pub available_date: Option<chrono::NaiveDateTime>,
    // pub has_trace_warehouse: bool,
    // pub update_date: chrono::NaiveDateTime,
    // pub is_synchronised: bool,
    // pub has_error: bool,
    // pub force_update: bool,
}

impl Model for ProductLegacyStagingModel {}

impl CanSelectAllModel for ProductLegacyStagingModel {
    fn select_all(connection: &mut DbConnection) -> Result<Vec<Self>, DieselError> {
        use crate::infrastructure::database::schema::legacy_staging::staging_product::dsl::*;

        staging_product
            .filter(id.is_not_null())
            .select(ProductLegacyStagingModel::as_select())
            .load(connection)
    }
}
pub struct ProductLegacyStagingDataSourceImpl;
impl ProductLegacyStagingDataSource for ProductLegacyStagingDataSourceImpl {
    type DbConnection = HasLegacyStagingConnection;

    fn find_all(&self) -> Result<Vec<ProductLegacyStagingModel>, DieselError> {
        ProductLegacyStagingModel::select_all(&mut Self::DbConnection::get_pooled_connection())
    }
}
pub(crate) trait ProductLegacyStagingDataSource {
    type DbConnection: HasConnection;
    fn find_all(&self) -> Result<Vec<ProductLegacyStagingModel>, DieselError>;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::infrastructure::database::connection::tests::{
        get_test_pooled_connection, reset_test_database,
    };
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_read_legacy_staging_source() {
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        let result =
            ProductLegacyStagingModel::select_all(&mut connection).expect("Error reading source");

        assert_eq!(result.len(), 3);
        assert_eq!(
            result[0],
            ProductLegacyStagingModel {
                id_source: 1,
                id: Some(11)
            }
        );
        assert_eq!(
            result[0],
            ProductLegacyStagingModel {
                id_source: 2,
                id: None
            }
        );
        assert_eq!(
            result[0],
            ProductLegacyStagingModel {
                id_source: 3,
                id: Some(33)
            }
        );
    }
}
