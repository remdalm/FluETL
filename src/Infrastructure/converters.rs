use crate::domain::{DomainError, Order};
use crate::infrastructure::csv_reader::CsvOrderDTO;

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

pub fn convert_to_orders(dtos: Vec<CsvOrderDTO>) -> Vec<Result<Order, DomainError>> {
    dtos.into_iter().map(|dto| dto.into()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fixtures::{csv_order_dto_fixtures, order_fixtures};

    #[test]
    fn test_convert_to_orders() {
        let dto_fixtures = csv_order_dto_fixtures();
        let results: Vec<Result<Order, DomainError>> = convert_to_orders(dto_fixtures.to_vec());

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

        let results: Vec<Result<Order, DomainError>> = convert_to_orders(dto_fixtures.to_vec());

        assert!(
            results[0].as_ref().is_err_and(|e| match e {
                DomainError::ValidationError(_) => true,
                _ => false,
            }),
            "Expected Domain Validation Error"
        );
    }
}
