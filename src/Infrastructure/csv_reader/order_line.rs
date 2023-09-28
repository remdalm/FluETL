use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CsvOrderLineDTO {
    pub c_orderline_id: String,
    pub c_order_id: String,
    pub item_ref: String,
    pub item_name: String,
    pub qty_ordered: String,
    pub qty_reserved: String,
    pub qty_delivered: String,
    pub due_date: String,
}

impl CsvDTO for CsvOrderLineDTO {}
