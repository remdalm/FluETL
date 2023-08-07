use super::infrastructure::date_format;
use chrono::NaiveDate;
use serde::Deserialize;

// Order entity
#[derive(Debug, Deserialize)]
pub struct Order {
    pub c_order_id: i32,
    pub c_bpartner_id: i32,
    pub name: String,
    #[serde(with = "date_format")]
    pub date: NaiveDate,
    pub order_ref: String,
    pub po_ref: String,
    pub origin: String,
    pub completion: i32,
    pub order_status: String,
    pub delivery_status: String,
}

// OrderLine entity
#[derive(Debug, Deserialize)]
pub struct OrderLine {
    pub c_orderline_id: i32,
    pub c_order_id: i32,
    pub product_ref: String,
    pub product_name: String,
    pub qty_ordered: i32,
    pub qty_reserved: i32,
    pub qty_delivered: i32,
    #[serde(with = "date_format")]
    pub due_date: NaiveDate,
}
