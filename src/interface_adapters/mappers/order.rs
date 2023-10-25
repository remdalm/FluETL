use std::env;

use crate::{
    domain::{
        dto::date_dto::{DateDTO, StringDateDTO},
        order::{Order, OrderDomainFactory},
        vo::completion::Completion,
    },
    infrastructure::{csv_reader::order::CsvOrderDTO, InfrastructureError},
};
use chrono::NaiveDateTime;

use crate::infrastructure::database::models::order::OrderModel;

use super::{convert_string_to_option_string, parse_string_to_u32, MappingError};

impl TryFrom<CsvOrderDTO> for OrderDomainFactory {
    type Error = MappingError;
    fn try_from(dto: CsvOrderDTO) -> Result<OrderDomainFactory, MappingError> {
        let date_format = env::var("CSV_DATE_FORMAT")
            .map_err(|e| MappingError::Infrastructure(InfrastructureError::EnvVarError(e)))?;

        let completion = convert_string_to_option_string(dto.completion)
            .map(Completion::try_from)
            .transpose()?;

        let date_dto = DateDTO::from(StringDateDTO::new(dto.date, date_format));
        Ok(OrderDomainFactory {
            order_id: parse_string_to_u32("c_order_id", &dto.c_order_id)?,
            client_id: parse_string_to_u32("c_bpartner_id", &dto.c_bpartner_id)?,
            client_name: convert_string_to_option_string(dto.client_name),
            date_dto,
            order_ref: dto.order_ref,
            po_ref: convert_string_to_option_string(dto.po_ref),
            origin: convert_string_to_option_string(dto.origin),
            completion,
            order_status: convert_string_to_option_string(dto.order_status),
        })
    }
}

impl From<Order> for OrderModel {
    fn from(order: Order) -> Self {
        Self {
            id_order: order.order_id(),
            id_client: order.client_id(),
            client_name: order.client_name().map(|s| s.to_string()),
            order_ref: order.order_ref().to_string(),
            po_ref: order.po_ref().map(|s| s.to_string()),
            completion: order.completion(),
            date: NaiveDateTime::new(
                order.date(),
                chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            ),
            origin: order.origin().map(|s| s.to_string()),
            order_status: order.order_status().map(|s| s.as_str().to_string()),
        }
    }
}

impl TryFrom<OrderModel> for Order {
    type Error = MappingError;
    fn try_from(order_model: OrderModel) -> Result<Order, MappingError> {
        OrderDomainFactory {
            order_id: order_model.id_order,
            client_id: order_model.id_client,
            client_name: order_model.client_name,
            order_ref: order_model.order_ref,
            date_dto: DateDTO::from(order_model.date.date()),
            po_ref: order_model.po_ref,
            origin: order_model.origin,
            completion: order_model.completion.map(Completion::from),
            order_status: order_model.order_status,
        }
        .make()
        .map_err(MappingError::Domain)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::order::{tests::order_fixtures, Order},
        infrastructure::{
            csv_reader::order::tests::csv_order_dto_fixtures,
            database::models::order::bench::order_model_fixtures,
        },
        interface_adapters::mappers::{
            convert_domain_entity_to_model, CsvEntityParser, MappingError, ModelToEntityParser,
        },
        tests::load_unit_test_env,
    };

    use super::*;

    struct ModelParser;
    struct CsvParser;
    impl ModelToEntityParser<OrderModel, Order> for ModelParser {}
    impl CsvEntityParser<CsvOrderDTO, Order> for CsvParser {
        fn transform_csv_row_to_entity(&self, csv: CsvOrderDTO) -> Result<Order, MappingError> {
            let factory: OrderDomainFactory = csv.try_into()?;
            factory.make().map_err(MappingError::Domain)
        }
    }

    #[test]
    fn test_convert_dtos_to_orders() {
        load_unit_test_env();
        let dto_fixtures = csv_order_dto_fixtures();
        let results: Vec<Result<Order, MappingError>> = CsvParser.parse_all(dto_fixtures.to_vec());

        let order_fixtures = order_fixtures();

        assert!(results[0].is_ok(), "Expected successful conversion");
        assert_eq!(results[0].as_ref().unwrap(), &order_fixtures[0]);

        assert!(results[1].is_ok(), "Expected successful conversion");
        assert_eq!(results[1].as_ref().unwrap(), &order_fixtures[1]);
    }

    #[test]
    fn test_convert_orders_to_models() {
        let models_fixtures = order_model_fixtures();
        let order_fixtures = order_fixtures();

        let results: Vec<OrderModel> = convert_domain_entity_to_model(order_fixtures.to_vec());

        assert_eq!(&results[0], &models_fixtures[0]);
        assert_eq!(&results[1], &models_fixtures[1]);
    }

    #[test]
    fn test_convert_models_to_orders() {
        let models_fixtures = order_model_fixtures();
        let order_fixtures = order_fixtures();

        let results = ModelParser.parse_all(models_fixtures.to_vec());

        for (i, result) in results.iter().enumerate() {
            assert!(result.is_ok(), "Expected successful conversion");
            assert_eq!(result.as_ref().unwrap(), &order_fixtures[i]);
        }
    }
}
