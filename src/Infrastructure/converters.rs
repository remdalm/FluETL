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
