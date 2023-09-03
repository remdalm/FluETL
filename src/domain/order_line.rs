use chrono::NaiveDate;

use super::{order::Order, DomainEntity, DomainError};

#[derive(Debug, Clone, PartialEq)]
pub struct OrderLine {
    order: Order,
    orderline_id: u32,
    item_ref: String,
    item_name: Option<String>,
    qty_ordered: u32,
    qty_reserved: u32,
    qty_delivered: u32,
    due_date: NaiveDate,
}

impl OrderLine {
    fn new(
        order: Order,
        orderline_id: u32,
        item_ref: String,
        item_name: Option<String>,
        qty_ordered: u32,
        qty_reserved: u32,
        qty_delivered: u32,
        due_date: NaiveDate,
    ) -> Result<Self, DomainError> {
        // Validation is performed here

        Ok(Self {
            order,
            orderline_id,
            item_ref,
            item_name,
            qty_ordered,
            qty_reserved,
            qty_delivered,
            due_date,
        })
    }
}

impl DomainEntity for OrderLine {}

pub struct OrderLineDomainFactory {
    pub order: Order,
    pub orderline_id: u32,
    pub item_ref: String,
    pub item_name: Option<String>,
    pub qty_ordered: u32,
    pub qty_reserved: u32,
    pub qty_delivered: u32,
    pub due_date: NaiveDate,
}

impl OrderLineDomainFactory {
    pub fn make(self) -> Result<OrderLine, DomainError> {
        OrderLine::new(
            self.order,
            self.orderline_id,
            self.item_ref,
            self.item_name,
            self.qty_ordered,
            self.qty_reserved,
            self.qty_delivered,
            self.due_date,
        )
    }
}
