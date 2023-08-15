use crate::{benches::OrderModel, domain::Order, infrastructure::csv_reader::CsvOrderDTO};

use super::*;

pub struct ImportOrderUseCase;

pub struct OrderManager;

impl UseCaseManager<CsvOrderDTO, Order, OrderModel> for OrderManager {}
impl CanReadCsvUseCase<CsvOrderDTO, Order> for OrderManager {}
impl CanPersistIntoDatabaseUseCase<Order, OrderModel> for OrderManager {}

impl UseCase<CsvOrderDTO, Order, OrderModel> for ImportOrderUseCase {
    type ManagerImpl = OrderManager;
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn
// }
