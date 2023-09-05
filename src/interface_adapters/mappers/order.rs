use std::env;

use crate::{
    domain::order::{Order, OrderDomainFactory, OrderEntityFromStringDTO, Origin},
    infrastructure::InfrastructureError,
};
use chrono::{Datelike, NaiveDate, NaiveDateTime};

use crate::infrastructure::{csv_reader::CsvOrderDTO, database::models::order::OrderModel};

use super::MappingError;

impl TryFrom<CsvOrderDTO> for Order {
    type Error = MappingError;
    fn try_from(dto: CsvOrderDTO) -> Result<Order, MappingError> {
        let date_format = env::var("CSV_DATE_FORMAT")
            .map_err(|e| MappingError::InfrastructureError(InfrastructureError::EnvVarError(e)))?;
        Order::new_from_sting_dto(
            OrderEntityFromStringDTO {
                c_order_id: dto.c_order_id,
                c_bpartner_id: dto.c_bpartner_id,
                client_name: dto.client_name,
                date: dto.date,
                order_ref: dto.order_ref,
                po_ref: dto.po_ref,
                origin: dto.origin,
                completion: dto.completion,
                order_status: dto.order_status,
                delivery_status: dto.delivery_status,
            },
            date_format.as_str(),
        )
        .map_err(|e| e.into())
    }
}

impl From<Order> for OrderModel {
    fn from(order: Order) -> Self {
        Self {
            id_order: order.c_order_id(),
            id_client: order.c_bpartner_id(),
            client_name: order.client_name().and_then(|s| Some(s.to_string())),
            order_ref: order.order_ref().to_string(),
            po_ref: order.po_ref().and_then(|s| Some(s.to_string())),
            completion: order.completion(),
            date: NaiveDateTime::new(
                order.date(),
                chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            ),
            origin: if let Origin::Unknown = order.origin() {
                None
            } else {
                Some(order.origin().to_string())
            },
            order_status: order.order_status().and_then(|s| Some(s.to_string())),
            delivery_status: order.delivery_status().and_then(|s| Some(s.to_string())),
        }
    }
}

impl TryFrom<OrderModel> for Order {
    type Error = MappingError;
    fn try_from(order_model: OrderModel) -> Result<Order, MappingError> {
        OrderDomainFactory {
            c_order_id: order_model.id_order,
            c_bpartner_id: order_model.id_client,
            client_name: order_model.client_name,
            order_ref: order_model.order_ref,
            date: NaiveDate::from_ymd_opt(
                order_model.date.year(),
                order_model.date.month(),
                order_model.date.day(),
            )
            .unwrap(),
            po_ref: order_model.po_ref,
            origin: Origin::from_optional_string(order_model.origin),
            completion: order_model.completion,
            order_status: order_model.order_status,
            delivery_status: order_model.delivery_status,
        }
        .make()
        .map_err(|e| MappingError::DomainError(e))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::order::Order,
        fixtures::{csv_order_dto_fixtures, order_fixtures, order_model_fixtures},
        interface_adapters::mappers::{
            convert_domain_entity_to_model, CSVToEntityParser, MappingError, ModelToEntityParser,
        },
        tests::load_unit_test_env,
    };

    use super::*;

    struct ModelParser;
    struct CsvParser;
    impl ModelToEntityParser<OrderModel, Order> for ModelParser {}
    impl CSVToEntityParser<CsvOrderDTO, Order> for CsvParser {
        fn transform_csv(&self, csv: CsvOrderDTO) -> Result<Order, MappingError> {
            csv.try_into()
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

    // No invalid data in fixtures
    // #[test]
    // fn test_convert_to_orders_with_errors() {
    //     // Simulate a CsvOrderDTO with invalid data for testing validation error

    //     load_unit_test_env();
    //     let mut dto_fixtures = csv_order_dto_fixtures();
    //     dto_fixtures[0].completion = "101".to_string();

    //     let results: Vec<Result<Order, MappingError>> =
    //         convert_csv_dto_to_domain_entity(dto_fixtures.to_vec());

    //     assert!(
    //         results[0].as_ref().is_err_and(|e| match e {
    //             MappingError::DomainError(DomainError::ValidationError(_)) => true,
    //             _ => false,
    //         }),
    //         "Expected Domain Validation Error"
    //     );
    // }

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
