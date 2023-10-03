use std::env;

use crate::{
    domain::delivery_slip::{DeliverySlip, DeliverySlipDomainFactory},
    infrastructure::{
        csv_reader::delivery_slip::CsvDeliverySlipDTO,
        database::models::delivery_slip::DeliverySlipModel, InfrastructureError,
    },
};

use super::{
    convert_string_to_option_date, convert_string_to_option_string, parse_string_to_u32,
    MappingError,
};

impl TryFrom<CsvDeliverySlipDTO> for DeliverySlipDomainFactory {
    type Error = MappingError;
    fn try_from(dto: CsvDeliverySlipDTO) -> Result<DeliverySlipDomainFactory, MappingError> {
        let date_format = env::var("CSV_DATE_FORMAT")
            .map_err(|e| MappingError::InfrastructureError(InfrastructureError::EnvVarError(e)))?;

        Ok(DeliverySlipDomainFactory {
            delivery_slip_id: parse_string_to_u32("delivery_slip_id", &dto.m_inout_id)?,
            client_id: parse_string_to_u32("c_bpartner_id", &dto.c_bpartner_id)?,
            reference: dto.documentno,
            shipping_date: convert_string_to_option_date(dto.shipping_date, &date_format)
                .transpose()?,
            po_ref: convert_string_to_option_string(dto.po_ref),
            carrier_name: convert_string_to_option_string(dto.carrier_name),
            trackingno: convert_string_to_option_string(dto.trackingno),
            status: convert_string_to_option_string(dto.status),
            tracking_link: convert_string_to_option_string(dto.tracking_link),
        })
    }
}

impl From<DeliverySlip> for DeliverySlipModel {
    fn from(delivery_slip: DeliverySlip) -> Self {
        Self {
            id_delivery_slip: delivery_slip.delivery_slip_id(),
            id_client: delivery_slip.client_id(),
            reference: delivery_slip.reference().to_string(),
            shipping_date: delivery_slip.shipping_date().map(|d| *d),
            po_ref: delivery_slip.po_ref().map(|s| s.to_string()),
            carrier_name: delivery_slip.carrier_name().map(|s| s.to_string()),
            status: delivery_slip.status().map(|s| s.to_string()),
            tracking_number: delivery_slip.trackingno().map(|s| s.to_string()),
            tracking_link: delivery_slip.tracking_link().map(|tl| tl.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::delivery_slip::tests::delivery_slip_fixtures,
        infrastructure::{
            csv_reader::delivery_slip::tests::csv_delivery_slip_dto_fixtures,
            database::models::delivery_slip::tests::delivery_slip_model_fixtures,
        },
        interface_adapters::mappers::{convert_domain_entity_to_model, CSVToEntityParser},
        tests::load_unit_test_env,
    };

    use super::*;

    struct CsvParser;
    impl CSVToEntityParser<CsvDeliverySlipDTO, DeliverySlip> for CsvParser {
        fn transform_csv(&self, csv: CsvDeliverySlipDTO) -> Result<DeliverySlip, MappingError> {
            let factory: DeliverySlipDomainFactory = csv.try_into()?;
            factory.make().map_err(|e| MappingError::DomainError(e))
        }
    }

    #[test]
    fn test_convert_csv_dtos_to_delivery_slips() {
        load_unit_test_env();

        let dto_fixtures = csv_delivery_slip_dto_fixtures();

        let results = CsvParser.parse_all(dto_fixtures.to_vec());

        let delivery_slip_fixtures = delivery_slip_fixtures();

        for (i, result) in results.iter().enumerate() {
            assert!(
                result.is_ok(),
                "Expected successful conversion for index {}",
                i
            );
            assert_eq!(result.as_ref().unwrap(), &delivery_slip_fixtures[i]);
        }
    }

    #[test]
    fn test_convert_delivery_slips_to_models() {
        let models_fixtures = delivery_slip_model_fixtures();
        let delivery_slip_fixtures = delivery_slip_fixtures();

        let results: Vec<DeliverySlipModel> =
            convert_domain_entity_to_model(delivery_slip_fixtures.to_vec());

        for (i, result) in results.iter().enumerate() {
            assert_eq!(result, &models_fixtures[i]);
        }
    }
}
