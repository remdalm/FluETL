use std::env;

use crate::{
    domain::{
        dto::date_dto::{DateDTO, StringDateDTO},
        invoice::{Invoice, InvoiceDomainFactory, InvoiceLocalizedTypeFactory},
        vo::{locale::Locale, Translation},
    },
    infrastructure::{
        csv_reader::invoice::{CsvInvoiceDTO, CsvInvoiceLocalizedItemDTO},
        database::models::invoice::{InvoiceLangModel, InvoiceModel},
        InfrastructureError,
    },
};

use super::{convert_string_to_option_string, parse_string_to_u32, MappingError};

impl TryFrom<CsvInvoiceDTO> for InvoiceDomainFactory {
    type Error = MappingError;
    fn try_from(dto: CsvInvoiceDTO) -> Result<InvoiceDomainFactory, MappingError> {
        let date_format = env::var("CSV_DATE_FORMAT")
            .map_err(|e| MappingError::Infrastructure(InfrastructureError::EnvVarError(e)))?;

        let date_dto = DateDTO::from(StringDateDTO::new(dto.date, date_format));

        Ok(InvoiceDomainFactory {
            invoice_id: parse_string_to_u32("invoice_id", &dto.c_invoice_id)?,
            client_id: parse_string_to_u32("c_bpartner_id", &dto.c_bpartner_id)?,
            client_name: convert_string_to_option_string(dto.client_name),
            invoice_ref: dto.invoice_ref,
            file_name: convert_string_to_option_string(dto.file_name),
            date_dto,
            po_ref: convert_string_to_option_string(dto.po_ref),
            invoice_types: Vec::new(),
            total_tax_excl: dto.total_tax_excl,
            total_tax_incl: dto.total_tax_incl,
        })
    }
}

impl TryFrom<CsvInvoiceLocalizedItemDTO> for InvoiceLocalizedTypeFactory {
    type Error = MappingError;
    fn try_from(
        dto: CsvInvoiceLocalizedItemDTO,
    ) -> Result<InvoiceLocalizedTypeFactory, MappingError> {
        Ok(InvoiceLocalizedTypeFactory {
            locale: Locale::try_from(dto.ad_language.as_str())?,
            type_name: Translation::new(dto.invoice_type)?,
            invoice_id: parse_string_to_u32("c_invoice_id", &dto.c_invoice_id)?,
        })
    }
}

impl From<Invoice> for (InvoiceModel, Vec<InvoiceLangModel>) {
    fn from(invoice: Invoice) -> Self {
        let invoice_types: Vec<InvoiceLangModel> = invoice
            .invoice_types()
            .iter()
            .map(|localized_item| InvoiceLangModel {
                id_invoice: invoice.invoice_id(),
                id_lang: localized_item.language().id(),
                type_name: localized_item.name().as_str().to_string(),
            })
            .collect();
        (
            InvoiceModel {
                id_invoice: invoice.invoice_id(),
                id_client: invoice.client_id(),
                client_name: invoice.client_name().map(|s| s.to_string()),
                invoice_ref: invoice.invoice_ref().to_string(),
                file_name: invoice.file_name(),
                date: *invoice.date(),
                po_ref: invoice.po_ref().map(|s| s.to_string()),
                total_tax_excl: invoice.total_tax_excl(),
                total_tax_incl: invoice.total_tax_incl(),
            },
            invoice_types,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        domain::{
            invoice::tests::invoice_fixtures,
            vo::localized_item::{tests::localized_item_fixtures, LocalizedItem},
        },
        infrastructure::{
            csv_reader::invoice::tests::csv_invoice_dto_fixtures,
            database::models::invoice::tests::{
                invoice_lang_model_fixtures, invoice_model_fixtures,
            },
        },
        interface_adapters::mappers::{
            convert_domain_entity_to_model, CsvEntityParser, MappingError,
        },
        tests::load_unit_test_env,
    };

    use super::*;

    struct CsvParser;
    impl CsvEntityParser<CsvInvoiceDTO, Invoice> for CsvParser {
        fn transform_csv_row_to_entity(&self, csv: CsvInvoiceDTO) -> Result<Invoice, MappingError> {
            let mut factory: InvoiceDomainFactory = csv.try_into()?;
            invoice_types_hashmap_fixture()
                .contains_key(&factory.invoice_id)
                .then(|| {
                    factory.invoice_types = invoice_types_hashmap_fixture()
                        .get(&factory.invoice_id)
                        .unwrap()
                        .to_owned();
                });
            factory.make().map_err(MappingError::Domain)
        }
    }

    fn invoice_types_hashmap_fixture() -> HashMap<u32, Vec<LocalizedItem>> {
        let mut invoice_type = HashMap::new();
        invoice_type.insert(
            1,
            vec![
                localized_item_fixtures()[0].clone(),
                localized_item_fixtures()[1].clone(),
            ],
        );
        invoice_type.insert(3, vec![localized_item_fixtures()[2].clone()]);
        invoice_type.insert(
            2,
            vec![
                localized_item_fixtures()[0].clone(),
                localized_item_fixtures()[1].clone(),
            ],
        );
        invoice_type
    }

    #[test]
    fn test_convert_dtos_to_invoices() {
        load_unit_test_env();
        let dto_fixtures = csv_invoice_dto_fixtures();
        let results: Vec<Result<Invoice, MappingError>> =
            CsvParser.parse_all(dto_fixtures.to_vec());

        let invoice_fixtures = invoice_fixtures();

        assert!(results[0].is_ok(), "Expected successful conversion");
        assert_eq!(results[0].as_ref().unwrap(), &invoice_fixtures[0]);

        assert!(results[2].is_ok(), "Expected successful conversion");
        assert_eq!(results[2].as_ref().unwrap(), &invoice_fixtures[1]);
    }

    #[test]
    fn test_convert_invoices_to_models() {
        let models_fixtures = invoice_model_fixtures();
        let model_lang_fixtures = invoice_lang_model_fixtures();
        let invoice_fixtures = invoice_fixtures();

        let results: Vec<(InvoiceModel, Vec<InvoiceLangModel>)> =
            convert_domain_entity_to_model(invoice_fixtures.to_vec());

        assert_eq!(&results[0].0, &models_fixtures[0]);
        assert_eq!(&results[1].0, &models_fixtures[1]);
        assert_eq!(&results[0].1, &model_lang_fixtures[0]);
        assert_eq!(&results[1].1, &model_lang_fixtures[1]);
    }
}
