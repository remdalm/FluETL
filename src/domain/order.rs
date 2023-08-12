use chrono::NaiveDate;

use super::DomainError;

// Order entity
#[derive(Debug)]
pub struct Order {
    c_order_id: u32,
    c_bpartner_id: u32,
    name: String,
    date: NaiveDate,
    order_ref: String,
    po_ref: String,
    origin: String,
    completion: u32,
    order_status: String,
    delivery_status: String,
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
        po_ref: String,
        origin: String,
        completion: u32,
        order_status: String,
        delivery_status: String,
    ) -> Result<Self, DomainError> {
        Self::validate_completion(completion)?;

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
    ) -> Result<Self, DomainError> {
        let c_order_id = c_order_id
            .parse::<u32>()
            .map_err(|err| DomainError::ParsingError(err.to_string()))?;
        let c_bpartner_id = c_bpartner_id
            .parse::<u32>()
            .map_err(|err| DomainError::ParsingError(err.to_string()))?;
        let date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d")
            .map_err(|err| DomainError::ParsingError(err.to_string()))?;
        let completion = completion
            .parse::<u32>()
            .map_err(|err| DomainError::ParsingError(err.to_string()))?;

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

    pub fn po_ref(&self) -> &str {
        self.po_ref.as_str()
    }

    pub fn origin(&self) -> &str {
        self.origin.as_str()
    }

    pub fn completion(&self) -> u32 {
        self.completion
    }

    pub fn order_status(&self) -> &str {
        self.order_status.as_str()
    }

    pub fn delivery_status(&self) -> &str {
        self.delivery_status.as_str()
    }
}
