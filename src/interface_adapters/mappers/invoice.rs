use std::env;

use crate::{
    domain::{
        dto::date_dto::{DateDTO, StringDateDTO},
        invoice::{Invoice, InvoiceDomainFactory},
    },
    infrastructure::{
        csv_reader::invoice::CsvInvoiceDTO, database::models::invoice::InvoiceModel,
        InfrastructureError,
    },
};

use super::{convert_string_to_option_string, parse_string_to_u32, MappingError};

impl<'a> TryFrom<CsvInvoiceDTO> for InvoiceDomainFactory {
    type Error = MappingError;
    fn try_from(dto: CsvInvoiceDTO) -> Result<InvoiceDomainFactory, MappingError> {
        let date_format = env::var("CSV_DATE_FORMAT")
            .map_err(|e| MappingError::InfrastructureError(InfrastructureError::EnvVarError(e)))?;

        let date_dto = DateDTO::from(StringDateDTO::new(dto.date, date_format));

        Ok(InvoiceDomainFactory {
            invoice_id: parse_string_to_u32("invoice_id", &dto.invoice_id)?,
            client_id: parse_string_to_u32("c_bpartner_id", &dto.c_bpartner_id)?,
            client_name: convert_string_to_option_string(dto.client_name),
            invoice_ref: dto.invoice_ref,
            file_name: convert_string_to_option_string(dto.file_name),
            date_dto,
            po_ref: convert_string_to_option_string(dto.po_ref),
            type_: dto.invoice_type,
            total_tax_excl: dto.total_tax_excl,
            total_tax_incl: dto.total_tax_incl,
        })
    }
}

impl From<Invoice> for InvoiceModel {
    fn from(invoice: Invoice) -> Self {
        Self {
            id_invoice: invoice.invoice_id(),
            id_client: invoice.client_id(),
            client_name: invoice.client_name().map(|s| s.to_string()),
            invoice_ref: invoice.invoice_ref().to_string(),
            file_name: invoice.file_name(),
            date: *invoice.date(),
            po_ref: invoice.po_ref().map(|s| s.to_string()),
            type_: invoice.type_().to_string(),
            total_tax_excl: invoice.total_tax_excl(),
            total_tax_incl: invoice.total_tax_incl(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::invoice::tests::invoice_fixtures,
        infrastructure::{
            csv_reader::invoice::tests::csv_invoice_dto_fixtures,
            database::models::invoice::tests::invoice_model_fixtures,
        },
        interface_adapters::mappers::{
            convert_domain_entity_to_model, CSVToEntityParser, MappingError,
        },
        tests::load_unit_test_env,
    };

    use super::*;

    struct CsvParser;
    impl CSVToEntityParser<CsvInvoiceDTO, Invoice> for CsvParser {
        fn transform_csv(&self, csv: CsvInvoiceDTO) -> Result<Invoice, MappingError> {
            let factory: InvoiceDomainFactory = csv.try_into()?;
            factory.make().map_err(MappingError::DomainError)
        }
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
        let invoice_fixtures = invoice_fixtures();

        let results: Vec<InvoiceModel> = convert_domain_entity_to_model(invoice_fixtures.to_vec());

        assert_eq!(&results[0], &models_fixtures[0]);
        assert_eq!(&results[1], &models_fixtures[1]);
    }
}
