use chrono::NaiveDate;

use super::{
    vo::{tracking_link::TrackingLink, Reference},
    DomainEntity, DomainError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DeliverySlip {
    delivery_slip_id: u32,
    c_bpartner_id: u32,
    reference: Reference,
    shipping_date: Option<NaiveDate>,
    po_ref: Option<String>,
    carrier_name: Option<String>,
    trackingno: Option<String>,
    status: Option<String>,
    tracking_link: Option<TrackingLink>,
}

impl DomainEntity for DeliverySlip {}

impl DeliverySlip {
    pub fn delivery_slip_id(&self) -> u32 {
        self.delivery_slip_id
    }

    pub fn c_bpartner_id(&self) -> u32 {
        self.c_bpartner_id
    }

    pub fn reference(&self) -> &str {
        &self.reference.as_str()
    }

    pub fn shipping_date(&self) -> Option<&NaiveDate> {
        self.shipping_date.as_ref()
    }

    pub fn po_ref(&self) -> Option<&str> {
        self.po_ref.as_deref()
    }

    pub fn carrier_name(&self) -> Option<&str> {
        self.carrier_name.as_deref()
    }

    pub fn trackingno(&self) -> Option<&str> {
        self.trackingno.as_deref()
    }

    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    pub fn tracking_link(&self) -> Option<&TrackingLink> {
        self.tracking_link.as_ref()
    }
}

pub struct DeliverySlipDomainFactory {
    pub delivery_slip_id: u32,
    pub c_bpartner_id: u32,
    pub reference: String,
    pub shipping_date: Option<NaiveDate>,
    pub po_ref: Option<String>,
    pub carrier_name: Option<String>,
    pub trackingno: Option<String>,
    pub status: Option<String>,
    pub tracking_link: Option<String>,
}

impl DeliverySlipDomainFactory {
    pub fn make(self) -> Result<DeliverySlip, DomainError> {
        let tracking_link = self
            .tracking_link
            .map(|tl| TrackingLink::try_from(tl))
            .transpose()
            .unwrap_or(None);
        Ok(DeliverySlip {
            delivery_slip_id: self.delivery_slip_id,
            c_bpartner_id: self.c_bpartner_id,
            reference: Reference::new(self.reference)?,
            shipping_date: self.shipping_date,
            po_ref: self.po_ref,
            carrier_name: self.carrier_name,
            trackingno: self.trackingno,
            status: self.status,
            tracking_link: tracking_link,
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub fn delivery_slip_fixtures() -> [DeliverySlip; 3] {
        [
            DeliverySlip {
                delivery_slip_id: 1,
                c_bpartner_id: 1,
                reference: Reference::new("Doc1".to_string()).unwrap(),
                shipping_date: Some(NaiveDate::from_ymd_opt(2023, 8, 1).unwrap()),
                po_ref: Some("PoRef1".to_string()),
                carrier_name: Some("Carrier1".to_string()),
                trackingno: Some("TrackingNo1".to_string()),
                status: Some("1".to_string()),
                tracking_link: Some(
                    TrackingLink::try_from("https://tracking1.com/123".to_string()).unwrap(),
                ),
            },
            DeliverySlip {
                delivery_slip_id: 2,
                c_bpartner_id: 2,
                reference: Reference::new("Doc2".to_string()).unwrap(),
                shipping_date: Some(NaiveDate::from_ymd_opt(2023, 8, 2).unwrap()),
                po_ref: Some("PoRef2".to_string()),
                carrier_name: Some("Carrier2".to_string()),
                trackingno: Some("TrackingNo2".to_string()),
                status: Some("2".to_string()),
                tracking_link: None,
            },
            DeliverySlip {
                delivery_slip_id: 3,
                c_bpartner_id: 1,
                reference: Reference::new("Doc3".to_string()).unwrap(),
                shipping_date: None,
                po_ref: None,
                carrier_name: None,
                trackingno: None,
                status: None,
                tracking_link: None,
            },
        ]
    }
}
