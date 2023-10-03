use chrono::NaiveDate;

use super::{order::Order, DomainEntity, DomainError, Validator};

#[derive(Debug, Clone, PartialEq)]
pub struct OrderLine {
    order: Order,
    orderline_id: u32,
    item_ref: String,
    item_name: Option<String>,
    qty_ordered: u32,
    qty_reserved: u32,
    qty_delivered: u32,
    due_date: Option<NaiveDate>,
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
        due_date: Option<NaiveDate>,
    ) -> Result<Self, DomainError> {
        // Validation is performed here
        Validator::string_is_not_empty("item_ref", &item_ref)?;

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

    pub fn order(&self) -> &Order {
        &self.order
    }

    pub fn orderline_id(&self) -> u32 {
        self.orderline_id
    }

    pub fn item_ref(&self) -> &str {
        &self.item_ref
    }

    pub fn item_name(&self) -> Option<&str> {
        self.item_name.as_deref()
    }

    pub fn qty_ordered(&self) -> u32 {
        self.qty_ordered
    }

    pub fn qty_reserved(&self) -> u32 {
        self.qty_reserved
    }

    pub fn qty_delivered(&self) -> u32 {
        self.qty_delivered
    }

    pub fn due_date(&self) -> Option<NaiveDate> {
        self.due_date
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
    pub due_date: Option<NaiveDate>,
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
    pub fn new_from_order(order: Order, fields: OrderLinePrimaryFields) -> Self {
        Self {
            order: order,
            orderline_id: fields.orderline_id,
            item_ref: fields.item_ref,
            item_name: fields.item_name,
            qty_ordered: fields.qty_ordered,
            qty_reserved: fields.qty_reserved,
            qty_delivered: fields.qty_delivered,
            due_date: fields.due_date,
        }
    }
}

#[derive(Debug)]
pub struct OrderLinePrimaryFields {
    pub order_id: u32,
    pub orderline_id: u32,
    pub item_ref: String,
    pub item_name: Option<String>,
    pub qty_ordered: u32,
    pub qty_reserved: u32,
    pub qty_delivered: u32,
    pub due_date: Option<NaiveDate>,
}

pub mod tests {
    use crate::domain::order::tests::order_fixtures;

    use super::*;
    #[allow(dead_code)] //use in other modules
    pub fn order_line_fixtures() -> [OrderLine; 3] {
        [
            OrderLineDomainFactory {
                order: order_fixtures()[0].clone(),
                orderline_id: 1,
                item_ref: "ItemRef1".to_string(),
                item_name: Some("ItemName1".to_string()),
                qty_ordered: 10,
                qty_reserved: 5,
                qty_delivered: 5,
                due_date: Some(NaiveDate::from_ymd_opt(2023, 8, 1).unwrap()),
            }
            .make()
            .unwrap(),
            OrderLineDomainFactory {
                order: order_fixtures()[0].clone(),
                orderline_id: 2,
                item_ref: "ItemRef2".to_string(),
                item_name: Some("ItemName2".to_string()),
                qty_ordered: 20,
                qty_reserved: 10,
                qty_delivered: 10,
                due_date: Some(NaiveDate::from_ymd_opt(2023, 8, 2).unwrap()),
            }
            .make()
            .unwrap(),
            OrderLineDomainFactory {
                order: order_fixtures()[1].clone(),
                orderline_id: 3,
                item_ref: "ItemRef3".to_string(),
                item_name: None,
                qty_ordered: 30,
                qty_reserved: 15,
                qty_delivered: 15,
                due_date: None,
            }
            .make()
            .unwrap(),
        ]
    }
}
