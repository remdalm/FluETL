use std::collections::HashMap;

use crate::{
    domain::{
        invoice::{Invoice, InvoiceDomainFactory, InvoiceLocalizedTypeFactory},
        vo::localized_item::LocalizedItem,
    },
    infrastructure::{
        csv_reader::{
            invoice::{CsvInvoiceDTO, CsvInvoiceLocalizedItemDTO},
            CanReadCSV, CsvType,
        },
        database::{
            batch::{Batch, Config},
            connection::{HasConnection, HasTargetConnection},
            models::invoice::{batch_upsert, InvoiceLangModel, InvoiceModel},
        },
    },
    interface_adapters::mappers::CsvEntityParser,
};

use super::{
    helpers::{
        csv::ImportFromSingleEntityBasedCsvUseCase, language::CanFetchLanguages,
        localized_item::ImportLocalizedItem, model::CanPersistIntoDatabaseUseCase,
    },
    *,
};

struct ImportInvoiceTypesUseCase;
impl CanReadCSV<CsvInvoiceLocalizedItemDTO> for ImportInvoiceTypesUseCase {
    fn find_all(&self) -> Result<Vec<CsvInvoiceLocalizedItemDTO>, InfrastructureError> {
        self.read(CsvType::InvoiceDocumentType)
    }
}
impl CanFetchLanguages for ImportInvoiceTypesUseCase {}
impl ImportLocalizedItem<InvoiceLocalizedTypeFactory, CsvInvoiceLocalizedItemDTO>
    for ImportInvoiceTypesUseCase
{
    fn source(&self) -> Result<Vec<CsvInvoiceLocalizedItemDTO>, UseCaseError> {
        self.find_all().map_err(|e| e.into())
    }
}

#[derive(Default)]
pub struct ImportInvoiceUseCase {
    invoice_types: HashMap<u32, Vec<LocalizedItem>>,
    batch: bool,
    batch_size: usize,
}

impl ImportInvoiceUseCase {
    pub fn new() -> Result<Self, Vec<UseCaseError>> {
        let invoice_types = ImportInvoiceTypesUseCase.make_localized_items()?;

        Ok(Self {
            invoice_types: ImportInvoiceTypesUseCase::group_localized_items(invoice_types),
            ..Self::default()
        })
    }
    pub fn set_batch(&mut self, batch_size: usize) {
        self.batch = true;
        self.batch_size = batch_size;
    }
}

impl CanReadCSV<CsvInvoiceDTO> for ImportInvoiceUseCase {
    fn find_all(&self) -> Result<Vec<CsvInvoiceDTO>, InfrastructureError> {
        self.read(CsvType::Invoice)
    }
}
impl CsvEntityParser<CsvInvoiceDTO, Invoice> for ImportInvoiceUseCase {
    fn transform_csv_row_to_entity(&self, csv: CsvInvoiceDTO) -> Result<Invoice, MappingError> {
        let mut factory: InvoiceDomainFactory = csv.try_into()?;
        self.invoice_types
            .contains_key(&factory.invoice_id)
            .then(|| {
                factory.invoice_types = self
                    .invoice_types
                    .get(&factory.invoice_id)
                    .unwrap()
                    .to_owned();
            });
        factory.make().map_err(MappingError::Domain)
    }
}
impl CanPersistIntoDatabaseUseCase<Invoice, (InvoiceModel, Vec<InvoiceLangModel>)>
    for ImportInvoiceUseCase
{
    type DbConnection = HasTargetConnection;
    fn set_batch<'a>(
        &'a self,
        models: &'a [(InvoiceModel, Vec<InvoiceLangModel>)],
    ) -> Option<Batch<(InvoiceModel, Vec<InvoiceLangModel>)>> {
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
impl
    ImportFromSingleEntityBasedCsvUseCase<
        CsvInvoiceDTO,
        Invoice,
        (InvoiceModel, Vec<InvoiceLangModel>),
    > for ImportInvoiceUseCase
{
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serial_test::serial;

    use super::*;
    use crate::infrastructure::database::connection::DbConnection;
    use crate::{
        domain::vo::localized_item::tests::localized_item_fixtures,
        infrastructure::database::{
            connection::tests::{
                get_test_pooled_connection, reset_test_database, HasTestConnection,
            },
            models::invoice::tests::{
                invoice_lang_model_fixtures, invoice_model_fixtures, read_invoices,
            },
        },
        infrastructure::{
            csv_reader::CsvType, database::models::invoice::tests::read_invoice_types,
        },
    };

    struct ImportInvoiceTypeUseCaseTest;
    impl CanReadCSV<CsvInvoiceLocalizedItemDTO> for ImportInvoiceTypeUseCaseTest {
        fn find_all(&self) -> Result<Vec<CsvInvoiceLocalizedItemDTO>, InfrastructureError> {
            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let csv_path = root_path
                .join("tests")
                .join("fixtures")
                .join("invoice_lang_for_unit_test.csv");
            self.read(CsvType::Test(csv_path))
        }
    }
    impl CanFetchLanguages for ImportInvoiceTypeUseCaseTest {}
    impl ImportLocalizedItem<InvoiceLocalizedTypeFactory, CsvInvoiceLocalizedItemDTO>
        for ImportInvoiceTypeUseCaseTest
    {
        // Mock method
        fn make_localized_items(&self) -> Result<Vec<(u32, LocalizedItem)>, Vec<UseCaseError>> {
            Ok(vec![
                (1, localized_item_fixtures()[0].clone()),
                (1, localized_item_fixtures()[1].clone()),
                (3, localized_item_fixtures()[2].clone()),
            ])
        }
        fn source(&self) -> Result<Vec<CsvInvoiceLocalizedItemDTO>, UseCaseError> {
            self.find_all().map_err(|e| e.into())
        }
    }

    #[derive(Default)]
    pub struct ImportInvoiceUseCaseTest {
        invoice_types: HashMap<u32, Vec<LocalizedItem>>,
        pub use_batch: bool,
    }
    impl ImportInvoiceUseCaseTest {
        pub fn new() -> Result<Self, Vec<UseCaseError>> {
            let invoice_types = ImportInvoiceTypeUseCaseTest.make_localized_items()?;

            Ok(Self {
                invoice_types: ImportInvoiceTypeUseCaseTest::group_localized_items(invoice_types),
                ..Self::default()
            })
        }
    }
    impl CanReadCSV<CsvInvoiceDTO> for ImportInvoiceUseCaseTest {
        fn find_all(&self) -> Result<Vec<CsvInvoiceDTO>, InfrastructureError> {
            // NamedTempFile is automatically deleted when it goes out of scope (this function ends)

            let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let csv_path = root_path
                .join("tests")
                .join("fixtures")
                .join("invoice_for_unit_test.csv");

            self.read(CsvType::Test(csv_path))
        }
    }
    impl CsvEntityParser<CsvInvoiceDTO, Invoice> for ImportInvoiceUseCaseTest {
        fn transform_csv_row_to_entity(&self, csv: CsvInvoiceDTO) -> Result<Invoice, MappingError> {
            let mut factory: InvoiceDomainFactory = csv.try_into()?;
            self.invoice_types
                .contains_key(&factory.invoice_id)
                .then(|| {
                    factory.invoice_types = self
                        .invoice_types
                        .get(&factory.invoice_id)
                        .unwrap()
                        .to_owned();
                });
            factory.make().map_err(MappingError::Domain)
        }
    }
    impl CanPersistIntoDatabaseUseCase<Invoice, (InvoiceModel, Vec<InvoiceLangModel>)>
        for ImportInvoiceUseCaseTest
    {
        type DbConnection = HasTestConnection;
        fn set_batch<'a>(
            &'a self,
            models: &'a [(InvoiceModel, Vec<InvoiceLangModel>)],
        ) -> Option<Batch<(InvoiceModel, Vec<InvoiceLangModel>)>> {
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
    impl
        ImportFromSingleEntityBasedCsvUseCase<
            CsvInvoiceDTO,
            Invoice,
            (InvoiceModel, Vec<InvoiceLangModel>),
        > for ImportInvoiceUseCaseTest
    {
    }

    #[test]
    #[serial]
    fn test_invoice_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // Result
        let use_case = ImportInvoiceUseCaseTest::new().unwrap();
        let errors = use_case.execute();

        assert_results(errors, &mut connection);
    }

    #[test]
    #[serial]
    fn test_batch_invoice_use_case() {
        // Arrange
        let mut connection = get_test_pooled_connection();
        reset_test_database(&mut connection);

        // Result
        let mut use_case = ImportInvoiceUseCaseTest::new().unwrap();
        use_case.use_batch = true;

        let errors = use_case.execute();

        assert_results(errors, &mut connection);
    }

    fn assert_results(errors: Option<Vec<UseCaseError>>, connection: &mut DbConnection) {
        assert!(
            errors.is_some_and(|errs| errs.len() == 1
                && format!("{:?}", errs[0])
                    == "Mapping(Domain(ValidationError(\"Invalid file name: INV -2.pdf\")))"),
            "Failed to execute use case: the test csv file contains 1 error",
        );

        let persisted_invoices = read_invoices(connection);
        assert_eq!(persisted_invoices.len(), 2);

        assert_eq!(persisted_invoices[0], invoice_model_fixtures()[0]);
        let invoice_items = read_invoice_types(connection, &persisted_invoices[0]);
        assert_eq!(invoice_items, invoice_lang_model_fixtures()[0]);

        //assert_eq!(persisted_invoices[1], invoice_model_fixtures()[1]);
        let invoice_items = read_invoice_types(connection, &persisted_invoices[1]);
        assert_eq!(invoice_items, invoice_lang_model_fixtures()[1]);
    }
}
