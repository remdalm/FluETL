use chrono::NaiveDate;

use super::{
    language::Language,
    order::Order,
    vo::{locale::Locale, localized_item::LocalizedItem, Reference, Translation},
    DomainEntity, DomainError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct OrderLine {
    order: Order,
    orderline_id: u32,
    item_ref: Reference,
    item_names: Vec<LocalizedItem>,
    qty_ordered: u32,
    qty_reserved: u32,
    qty_delivered: u32,
    due_date: Option<NaiveDate>,
}

impl OrderLine {
    pub fn order(&self) -> &Order {
        &self.order
    }

    pub fn orderline_id(&self) -> u32 {
        self.orderline_id
    }

    pub fn item_ref(&self) -> &str {
        self.item_ref.as_str()
    }

    pub fn item_names(&self) -> &[LocalizedItem] {
        &self.item_names
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
    pub item_names: Vec<LocalizedItem>,
    pub qty_ordered: u32,
    pub qty_reserved: u32,
    pub qty_delivered: u32,
    pub due_date: Option<NaiveDate>,
}

impl OrderLineDomainFactory {
    pub fn make(self) -> Result<OrderLine, DomainError> {
        Ok(OrderLine {
            order: self.order,
            orderline_id: self.orderline_id,
            item_ref: Reference::new(self.item_ref)?,
            item_names: self.item_names,
            qty_ordered: self.qty_ordered,
            qty_reserved: self.qty_reserved,
            qty_delivered: self.qty_delivered,
            due_date: self.due_date,
        })
    }
    pub fn new_from_order(order: Order, fields: &OrderLinePrimaryFields) -> Self {
        Self {
            order,
            orderline_id: fields.orderline_id,
            item_ref: fields.item_ref.clone(),
            item_names: Vec::new(),
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
    pub qty_ordered: u32,
    pub qty_reserved: u32,
    pub qty_delivered: u32,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug)]
pub struct OrderLineLocalizedItemFactory {
    pub orderline_id: u32,
    pub locale: Locale,
    pub name: Translation,
}

impl OrderLineLocalizedItemFactory {
    pub fn make_from_language(&self, language: &Language) -> Result<LocalizedItem, DomainError> {
        if language.locale() != &self.locale {
            return Err(DomainError::ValidationError(format!(
                "Language locale {} does not match item locale {}",
                language.locale().as_str(),
                self.locale.as_str()
            )));
        }
        // BIG REFACTORING TODO: Use &Language instead of Language
        Ok(LocalizedItem::new(language.clone(), self.name.clone()))
    }
}

pub mod tests {
    use crate::domain::{
        order::tests::order_fixtures, vo::localized_item::tests::localized_item_fixtures,
    };

    use super::*;
    #[allow(dead_code)] //use in other modules
    pub fn order_line_fixtures() -> [OrderLine; 3] {
        [
            OrderLine {
                order: order_fixtures()[0].clone(),
                orderline_id: 1,
                item_ref: Reference::new("ItemRef1".to_string()).unwrap(),
                item_names: vec![
                    localized_item_fixtures()[0].clone(),
                    localized_item_fixtures()[1].clone(),
                ],
                qty_ordered: 10,
                qty_reserved: 5,
                qty_delivered: 5,
                due_date: Some(NaiveDate::from_ymd_opt(2023, 8, 1).unwrap()),
            },
            OrderLine {
                order: order_fixtures()[0].clone(),
                orderline_id: 2,
                item_ref: Reference::new("ItemRef2".to_string()).unwrap(),
                item_names: vec![localized_item_fixtures()[2].clone()],
                qty_ordered: 20,
                qty_reserved: 10,
                qty_delivered: 10,
                due_date: Some(NaiveDate::from_ymd_opt(2023, 8, 2).unwrap()),
            },
            OrderLine {
                order: order_fixtures()[1].clone(),
                orderline_id: 3,
                item_ref: Reference::new("ItemRef3".to_string()).unwrap(),
                item_names: Vec::new(),
                qty_ordered: 30,
                qty_reserved: 15,
                qty_delivered: 15,
                due_date: None,
            },
        ]
    }
}
