use chrono::NaiveDateTime;

use crate::domain::{DomainEntity, DomainError, MappingClient, Order};
use crate::infrastructure::csv_reader::{CsvMappingClientDTO, CsvOrderDTO};

use super::database::models::order::OrderModel;
use super::database::models::Model;

impl From<CsvOrderDTO> for Result<Order, DomainError> {
    fn from(dto: CsvOrderDTO) -> Result<Order, DomainError> {
        Order::new_from_string(
            dto.c_order_id,
            dto.c_bpartner_id,
            dto.name,
            dto.date,
            dto.order_ref,
            dto.po_ref,
            dto.origin,
            dto.completion,
            dto.order_status,
            dto.delivery_status,
        )
    }
}

impl From<CsvMappingClientDTO> for Result<MappingClient, DomainError> {
    fn from(dto: CsvMappingClientDTO) -> Result<MappingClient, DomainError> {
        MappingClient::new_from_string(dto.c_bpartner_id, dto.ad_user_id)
    }
}

pub fn convert_csv_dto_to_domain_entity<CSV, DE>(dtos: Vec<CSV>) -> Vec<Result<DE, DomainError>>
where
    CSV: Into<Result<DE, DomainError>>,
    DE: DomainEntity,
{
    dtos.into_iter().map(|dto| dto.into()).collect()
}

impl From<Order> for OrderModel {
    fn from(order: Order) -> Self {
        Self {
            id_order: order.c_order_id(),
            id_client: order.c_bpartner_id(),
            order_ref: order.order_ref().to_string(),
            date: NaiveDateTime::new(
                order.date(),
                chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            ),
            // order_status: None,
            // delivery_status: None,
            order_status: order.order_status().and_then(|s| Some(s.to_string())),
            delivery_status: order.delivery_status().and_then(|s| Some(s.to_string())),
        }
    }
}

pub fn convert_domain_entity_to_model<DE, M>(d: Vec<DE>) -> Vec<M>
where
    DE: DomainEntity + Into<M>,
    M: Model,
{
    d.into_iter().map(|de| de.into()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::{csv_order_dto_fixtures, order_fixtures, order_model_fixtures};

    #[test]
    fn test_convert_dtos_to_orders() {
        let dto_fixtures = csv_order_dto_fixtures();
        let results: Vec<Result<Order, DomainError>> =
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
        let mut dto_fixtures = csv_order_dto_fixtures();
        dto_fixtures[0].completion = "101".to_string();

        let results: Vec<Result<Order, DomainError>> =
            convert_csv_dto_to_domain_entity(dto_fixtures.to_vec());

        assert!(
            results[0].as_ref().is_err_and(|e| match e {
                DomainError::ValidationError(_) => true,
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
