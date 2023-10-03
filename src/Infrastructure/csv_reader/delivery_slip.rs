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

#[cfg(test)]
pub mod tests {
    use super::*;
    pub fn csv_delivery_slip_dto_fixtures() -> [CsvDeliverySlipDTO; 3] {
        [
            CsvDeliverySlipDTO {
                m_inout_id: 1.to_string(),
                c_bpartner_id: 1.to_string(),
                documentno: "Doc1".to_string(),
                shipping_date: "2023-08-01".to_string(),
                po_ref: "PoRef1".to_string(),
                carrier_name: "Carrier1".to_string(),
                trackingno: "TrackingNo1".to_string(),
                status: "1".to_string(),
                tracking_link: "https://tracking1.com/123".to_string(),
            },
            CsvDeliverySlipDTO {
                m_inout_id: 2.to_string(),
                c_bpartner_id: 2.to_string(),
                documentno: "Doc2".to_string(),
                shipping_date: "2023-08-02".to_string(),
                po_ref: "PoRef2".to_string(),
                carrier_name: "Carrier2".to_string(),
                trackingno: "TrackingNo2".to_string(),
                status: "2".to_string(),
                tracking_link: "http:://tracking2.com".to_string(),
            },
            CsvDeliverySlipDTO {
                m_inout_id: 3.to_string(),
                c_bpartner_id: 1.to_string(),
                documentno: "Doc3".to_string(),
                shipping_date: String::new(),
                po_ref: String::new(),
                carrier_name: String::new(),
                trackingno: String::new(),
                status: String::new(),
                tracking_link: String::new(),
            },
        ]
    }
}
