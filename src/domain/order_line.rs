use chrono::NaiveDate;

// OrderLine entity
#[derive(Debug)]
pub struct OrderLine {
    pub c_orderline_id: u32,
    pub c_order_id: u32,
    pub product_ref: String,
    pub product_client_name: String,
    pub qty_ordered: u32,
    pub qty_reserved: u32,
    pub qty_delivered: u32,
    pub due_date: NaiveDate,
}
