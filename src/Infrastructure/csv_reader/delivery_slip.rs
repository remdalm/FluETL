use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CsvDeliverySlipDTO {
    pub m_inout_id: String,
    pub c_bpartner_id: String,
    pub documentno: String,
    pub shipping_date: String,
    pub po_ref: String,
    pub carrier_name: String,
    pub trackingno: String,
    pub status: String,
    pub tracking_link: String,
}

impl CsvDTO for CsvDeliverySlipDTO {}
