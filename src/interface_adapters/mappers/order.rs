use std::env;

use crate::{
    domain::order::{Order, OrderEntityFromStringDTO, Origin},
    infrastructure::InfrastructureError,
};
use chrono::NaiveDateTime;

use crate::infrastructure::{csv_reader::CsvOrderDTO, database::models::order::OrderModel};

use super::MappingError;

impl From<CsvOrderDTO> for Result<Order, MappingError> {
    fn from(dto: CsvOrderDTO) -> Result<Order, MappingError> {
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

#[cfg(test)]
mod tests {
    use crate::{
        domain::{order::Order, DomainError},
        fixtures::{csv_order_dto_fixtures, order_fixtures, order_model_fixtures},
        infrastructure::database::models::order::OrderModel,
        interface_adapters::mappers::{
            convert_csv_dto_to_domain_entity, convert_domain_entity_to_model, MappingError,
        },
        tests::load_unit_test_env,
    };

    #[test]
    fn test_convert_dtos_to_orders() {
        load_unit_test_env();
        let dto_fixtures = csv_order_dto_fixtures();
        let results: Vec<Result<Order, MappingError>> =
            convert_csv_dto_to_domain_entity(dto_fixtures.to_vec());

        let order_fixtures = order_fixtures();

        assert!(results[0].is_ok(), "Expected successful conversion");
        assert_eq!(results[0].as_ref().unwrap(), &order_fixtures[0]);

        assert!(results[1].is_ok(), "Expected successful conversion");
        assert_eq!(results[1].as_ref().unwrap(), &order_fixtures[1]);
    }

    #[test]
    fn test_convert_to_orders_with_errors() {
        // Simulate a CsvOrderDTO with invalid data for testing validation error

        load_unit_test_env();
        let mut dto_fixtures = csv_order_dto_fixtures();
        dto_fixtures[0].completion = "101".to_string();

        let results: Vec<Result<Order, MappingError>> =
            convert_csv_dto_to_domain_entity(dto_fixtures.to_vec());

        assert!(
            results[0].as_ref().is_err_and(|e| match e {
                MappingError::DomainError(DomainError::ValidationError(_)) => true,
                _ => false,
            }),
            "Expected Domain Validation Error"
        );
    }

    #[test]
    fn test_convert_orders_to_models() {
        let models_fixtures = order_model_fixtures();
        let order_fixtures = order_fixtures();

        let results: Vec<OrderModel> = convert_domain_entity_to_model(order_fixtures.to_vec());

        assert_eq!(&results[0], &models_fixtures[0]);
        assert_eq!(&results[1], &models_fixtures[1]);
    }
}
