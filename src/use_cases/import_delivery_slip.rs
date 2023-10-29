use crate::{
    domain::delivery_slip::{DeliverySlip, DeliverySlipDomainFactory},
    infrastructure::{
        csv_reader::{delivery_slip::CsvDeliverySlipDTO, CanReadCSV, CsvType},
        database::{
            batch::{Batch, Config},
            connection::{HasConnection, HasTargetConnection},
            models::delivery_slip::{batch_upsert, DeliverySlipModel},
        },
    },
    interface_adapters::mappers::CsvEntityParser,
};

use super::{
    helpers::{csv::ImportFromSingleEntityBasedCsvUseCase, model::CanPersistIntoDatabaseUseCase},
    *,
};

#[derive(Default)]
pub struct ImportDeliverySlipUseCase {
    batch: bool,
    batch_size: usize,
}

impl ImportDeliverySlipUseCase {
    pub fn set_batch(&mut self, batch_size: usize) {
        self.batch = true;
        self.batch_size = batch_size;
    }
}

impl CanReadCSV<CsvDeliverySlipDTO> for ImportDeliverySlipUseCase {
    fn find_all(&self) -> Result<Vec<CsvDeliverySlipDTO>, InfrastructureError> {
        self.read(CsvType::DeliverySlip)
    }
}
impl CsvEntityParser<CsvDeliverySlipDTO, DeliverySlip> for ImportDeliverySlipUseCase {
    fn transform_csv_row_to_entity(
        &self,
        csv: CsvDeliverySlipDTO,
    ) -> Result<DeliverySlip, MappingError> {
        let factory: DeliverySlipDomainFactory = csv.try_into()?;
        factory.make().map_err(MappingError::Domain)
    }
}
impl CanPersistIntoDatabaseUseCase<DeliverySlip, DeliverySlipModel> for ImportDeliverySlipUseCase {
    type DbConnection = HasTargetConnection;
    fn set_batch<'a>(
        &'a self,
        models: &'a [DeliverySlipModel],
    ) -> Option<Batch<DeliverySlipModel>> {
        if self.batch {
            Some(Batch::new(
                models,
                Some(Config::new(self.batch_size)),
                batch_upsert,
                HasTargetConnection::get_pooled_connection(),
            ))
        } else {
            None
        }
    }
}
impl ImportFromSingleEntityBasedCsvUseCase<CsvDeliverySlipDTO, DeliverySlip, DeliverySlipModel>
    for ImportDeliverySlipUseCase
{
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serial_test::serial;

    use super::*;
    use crate::{
        infrastructure::database::connection::tests::{
            get_test_pooled_connection, reset_test_database, HasTestConnection,
        },
        infrastructure::{
            csv_reader::CsvType,
            database::models::delivery_slip::tests::{
                delivery_slip_model_fixtures, read_delivery_slips,
            },
        },
    };

    #[derive(Default)]
    pub struct ImportDeliverySlipUseCaseTest {
        pub use_batch: bool,
    }
    impl CanReadCSV<CsvDeliverySlipDTO> for ImportDeliverySlipUseCaseTest {
        fn find_all(&self) -> Result<Vec<CsvDeliverySlipDTO>, InfrastructureError> {
            // NamedTempFile is automatically deleted when it goes out of scope (this function ends)

            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let csv_path = root_path
                .join("tests")
                .join("fixtures")
                .join("delivery_slip_for_unit_test.csv");

            self.read(CsvType::Test(csv_path))
        }
    }
    impl CsvEntityParser<CsvDeliverySlipDTO, DeliverySlip> for ImportDeliverySlipUseCaseTest {
        fn transform_csv_row_to_entity(
            &self,
            csv: CsvDeliverySlipDTO,
        ) -> Result<DeliverySlip, MappingError> {
            let factory: DeliverySlipDomainFactory = csv.try_into()?;
            factory.make().map_err(MappingError::Domain)
        }
    }
    impl CanPersistIntoDatabaseUseCase<DeliverySlip, DeliverySlipModel>
        for ImportDeliverySlipUseCaseTest
    {
        type DbConnection = HasTestConnection;
        fn set_batch<'a>(
            &'a self,
            models: &'a [DeliverySlipModel],
        ) -> Option<Batch<DeliverySlipModel>> {
            if self.use_batch {
                return Some(Batch::new(
                    models,
                    None,
                    batch_upsert,
                    HasTestConnection::get_pooled_connection(),
                ));
            }
            None
        }
    }
    impl ImportFromSingleEntityBasedCsvUseCase<CsvDeliverySlipDTO, DeliverySlip, DeliverySlipModel>
        for ImportDeliverySlipUseCaseTest
    {
    }

    #[test]
    #[serial]
    fn test_delivery_slip_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // Result
        let use_case = ImportDeliverySlipUseCaseTest::default();
        let errors = use_case.execute();

        // Assert
        assert!(
            errors.is_none(),
            "Failed to execute use case: {:?}",
            errors.unwrap()
        );
        let persisted_delivery_slips = read_delivery_slips(&mut connection);
        assert_eq!(persisted_delivery_slips.len(), 3);
        assert_eq!(
            persisted_delivery_slips[0],
            delivery_slip_model_fixtures()[0]
        );
        assert_eq!(
            persisted_delivery_slips[1],
            delivery_slip_model_fixtures()[1]
        );
    }

    #[test]
    #[serial]
    fn test_batch_delivery_slip_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // Result
        let use_case = ImportDeliverySlipUseCaseTest { use_batch: true };
        let errors = use_case.execute();

        // Assert
        assert!(errors.is_none(), "Failed to execute use case");
        let delivery_slips = read_delivery_slips(&mut connection);
        assert_eq!(delivery_slips.len(), 3);
        for (i, persisted_order_line) in delivery_slips.iter().enumerate() {
            assert_eq!(*persisted_order_line, delivery_slip_model_fixtures()[i]);
        }
    }
}
