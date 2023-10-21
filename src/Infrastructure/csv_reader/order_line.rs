use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CsvOrderLineDTO {
    pub c_orderline_id: String,
    pub c_order_id: String,
    pub item_ref: String,
    pub qty_ordered: String,
    pub qty_reserved: String,
    pub qty_delivered: String,
    pub due_date: String,
}

impl CsvDTO for CsvOrderLineDTO {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CsvOrderLineLocalizedItemDTO {
    pub c_orderline_id: String,
    pub ad_language: String,
    pub item_name: String,
}

impl CsvDTO for CsvOrderLineLocalizedItemDTO {}

pub mod tests {
    use super::*;
    #[allow(dead_code)] //use in other modules
    pub fn csv_order_line_dto_fixtures() -> [CsvOrderLineDTO; 3] {
        [
            CsvOrderLineDTO {
                c_orderline_id: 1.to_string(),
                c_order_id: 1.to_string(),
                item_ref: "ItemRef1".to_string(),
                qty_ordered: "10".to_string(),
                qty_reserved: "5".to_string(),
                qty_delivered: "5".to_string(),
                due_date: "2023-08-01".to_string(),
            },
            CsvOrderLineDTO {
                c_orderline_id: 2.to_string(),
                c_order_id: 1.to_string(),
                item_ref: "ItemRef2".to_string(),
                qty_ordered: "20".to_string(),
                qty_reserved: "10".to_string(),
                qty_delivered: "10".to_string(),
                due_date: "2023-08-02".to_string(),
            },
            CsvOrderLineDTO {
                c_orderline_id: 3.to_string(),
                c_order_id: 2.to_string(),
                item_ref: "ItemRef3".to_string(),
                qty_ordered: "30".to_string(),
                qty_reserved: "15".to_string(),
                qty_delivered: "15".to_string(),
                due_date: String::new(),
            },
        ]
    }
}
