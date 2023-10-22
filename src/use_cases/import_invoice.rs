use crate::{
    domain::invoice::{Invoice, InvoiceDomainFactory},
    infrastructure::{
        csv_reader::{invoice::CsvInvoiceDTO, CsvType},
        database::{
            batch::{Batch, Config},
            connection::{HasConnection, HasTargetConnection},
            models::invoice::{batch_upsert, InvoiceModel},
        },
    },
    interface_adapters::mappers::CSVToEntityParser,
};

use super::{
    helpers::{
        csv::{CanReadCsvUseCase, ImportCsvUseCase},
        model::CanPersistIntoDatabaseUseCase,
    },
    *,
};

#[derive(Default)]
pub struct ImportInvoiceUseCase {
    batch: bool,
    batch_size: usize,
}

impl ImportInvoiceUseCase {
    pub fn set_batch(&mut self, batch_size: usize) {
        self.batch = true;
        self.batch_size = batch_size;
    }
}

impl CanReadCsvUseCase<CsvInvoiceDTO> for ImportInvoiceUseCase {}
impl CSVToEntityParser<CsvInvoiceDTO, Invoice> for ImportInvoiceUseCase {
    fn transform_csv(&self, csv: CsvInvoiceDTO) -> Result<Invoice, MappingError> {
        let factory: InvoiceDomainFactory = csv.try_into()?;
        factory.make().map_err(MappingError::Domain)
    }
}
impl CanPersistIntoDatabaseUseCase<Invoice, InvoiceModel> for ImportInvoiceUseCase {
    type DbConnection = HasTargetConnection;
    fn set_batch<'a>(&'a self, models: &'a [InvoiceModel]) -> Option<Batch<InvoiceModel>> {
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
impl ImportCsvUseCase<CsvInvoiceDTO, Invoice, InvoiceModel> for ImportInvoiceUseCase {
    fn get_csv_type(&self) -> CsvType {
        CsvType::Invoice
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{
        infrastructure::csv_reader::CsvType,
        infrastructure::database::{
            connection::tests::{
                get_test_pooled_connection, reset_test_database, HasTestConnection,
            },
            models::invoice::tests::{invoice_model_fixtures, read_invoices},
        },
    };

    pub struct ImportInvoiceUseCaseTest;
    impl CanReadCsvUseCase<CsvInvoiceDTO> for ImportInvoiceUseCaseTest {}
    impl CSVToEntityParser<CsvInvoiceDTO, Invoice> for ImportInvoiceUseCaseTest {
        fn transform_csv(&self, csv: CsvInvoiceDTO) -> Result<Invoice, MappingError> {
            let factory: InvoiceDomainFactory = csv.try_into()?;
            factory.make().map_err(MappingError::Domain)
        }
    }
    impl CanPersistIntoDatabaseUseCase<Invoice, InvoiceModel> for ImportInvoiceUseCaseTest {
        type DbConnection = HasTestConnection;
    }
    impl ImportCsvUseCase<CsvInvoiceDTO, Invoice, InvoiceModel> for ImportInvoiceUseCaseTest {
        fn get_csv_type(&self) -> CsvType {
            // NamedTempFile is automatically deleted when it goes out of scope (this function ends)

            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let csv_path = root_path
                .join("tests")
                .join("fixtures")
                .join("invoice_for_unit_test.csv");

            CsvType::Test(csv_path)
        }
    }

    #[test]
    fn test_invoice_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // Result
        let use_case = ImportInvoiceUseCaseTest;
        let errors = use_case.execute();

        // Assert
        assert!(errors.is_some_and(|v| v.len() == 1));
        let persisted_invoices = read_invoices(&mut connection);
        assert_eq!(persisted_invoices.len(), 2);
        assert_eq!(persisted_invoices[0], invoice_model_fixtures()[0]);
        assert_eq!(persisted_invoices[1], invoice_model_fixtures()[1]);
    }

    // TODO: Test with failure
}
