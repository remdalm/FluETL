use chrono::NaiveDate;

use super::{convert_string_to_option_string, DomainEntity, DomainError};

// Order entity
#[derive(Debug, Clone)]
pub struct Order {
    c_order_id: u32,
    c_bpartner_id: u32,
    name: String,
    date: NaiveDate,
    order_ref: String,
    po_ref: Option<String>,
    origin: String,
    completion: Option<u32>,
    order_status: Option<String>,
    delivery_status: Option<String>,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.c_order_id == other.c_order_id
            && self.c_bpartner_id == other.c_bpartner_id
            && self.name == other.name
            && self.date == other.date
            && self.order_ref == other.order_ref
            && self.po_ref == other.po_ref
            && self.origin == other.origin
            && self.completion == other.completion
            && self.order_status == other.order_status
            && self.delivery_status == other.delivery_status
    }
}

impl Order {
    pub fn new(
        c_order_id: u32,
        c_bpartner_id: u32,
        name: String,
        date: NaiveDate,
        order_ref: String,
        po_ref: Option<String>,
        origin: String,
        completion: Option<u32>,
        order_status: Option<String>,
        delivery_status: Option<String>,
    ) -> Result<Self, DomainError> {
        if completion.is_some() {
            Self::validate_completion(completion.unwrap())?;
        }

        Ok(Self {
            c_order_id,
            c_bpartner_id,
            name,
            date,
            order_ref,
            po_ref,
            origin,
            completion,
            order_status,
            delivery_status,
        })
    }

    pub fn new_from_string(
        c_order_id: String,
        c_bpartner_id: String,
        name: String,
        date: String,
        order_ref: String,
        po_ref: String,
        origin: String,
        completion: String,
        order_status: String,
        delivery_status: String,
        date_format: &str,
    ) -> Result<Self, DomainError> {
        let c_order_id = c_order_id.parse::<u32>().map_err(|err| {
            DomainError::ParsingError(
                err.to_string() + format!(": c_order_id => {}", c_order_id).as_str(),
            )
        })?;
        let c_bpartner_id = c_bpartner_id.parse::<u32>().map_err(|err| {
            DomainError::ParsingError(
                err.to_string() + format!(": c_bpartner_id => {}", c_bpartner_id).as_str(),
            )
        })?;
        let date = NaiveDate::parse_from_str(date.as_str(), date_format).map_err(|err| {
            DomainError::ParsingError(err.to_string() + format!(": date => {}", date).as_str())
        })?;

        let completion = convert_string_to_option_string(completion)
            .and_then(|s| {
                Some(s.replace("%", "").parse::<u32>().map_err(|err| {
                    DomainError::ParsingError(
                        err.to_string() + format!(": completion => {}", s).as_str(),
                    )
                }))
            })
            .transpose()?;

        let po_ref = convert_string_to_option_string(po_ref);
        let order_status = convert_string_to_option_string(order_status);
        let delivery_status = convert_string_to_option_string(delivery_status);

        Self::new(
            c_order_id,
            c_bpartner_id,
            name,
            date,
            order_ref,
            po_ref,
            origin,
            completion,
            order_status,
            delivery_status,
        )
    }

    fn validate_completion(completion: u32) -> Result<(), DomainError> {
        if completion > 100 {
            Err(DomainError::ValidationError(
                "Completion must be an integer between 0 and 100".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    // Getters
    pub fn c_order_id(&self) -> u32 {
        self.c_order_id
    }

    pub fn c_bpartner_id(&self) -> u32 {
        self.c_bpartner_id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn order_ref(&self) -> &str {
        self.order_ref.as_str()
    }

    pub fn po_ref(&self) -> Option<&str> {
        self.po_ref.as_deref()
    }

    pub fn origin(&self) -> &str {
        self.origin.as_str()
    }

    pub fn completion(&self) -> Option<u32> {
        self.completion
    }

    pub fn order_status(&self) -> Option<&str> {
        self.order_status.as_deref()
    }

    pub fn delivery_status(&self) -> Option<&str> {
        self.delivery_status.as_deref()
    }
}

impl DomainEntity for Order {}
