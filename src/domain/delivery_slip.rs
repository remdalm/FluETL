use chrono::NaiveDate;

use super::{new_type::Reference, vo::tracking_link::TrackingLink, DomainEntity, DomainError};

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
    pub fn new(
        delivery_slip_id: u32,
        c_bpartner_id: u32,
        reference: Reference,
        shipping_date: Option<NaiveDate>,
        po_ref: Option<String>,
        carrier_name: Option<String>,
        trackingno: Option<String>,
        status: Option<String>,
        tracking_link: Option<TrackingLink>,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            delivery_slip_id,
            c_bpartner_id,
            reference,
            shipping_date,
            po_ref,
            carrier_name,
            trackingno,
            status,
            tracking_link,
        })
    }

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
        DeliverySlip::new(
            self.delivery_slip_id,
            self.c_bpartner_id,
            Reference::new(self.reference)?,
            self.shipping_date,
            self.po_ref,
            self.carrier_name,
            self.trackingno,
            self.status,
            tracking_link,
        )
    }
}
