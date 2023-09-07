use chrono::NaiveDate;

use crate::interface_adapters::mappers::convert_string_to_option_string;

use super::{DomainEntity, DomainError};

#[derive(Debug, Clone, PartialEq)]
pub enum Origin {
    Web,
    EDI,
    Unknown,
}

impl ToString for Origin {
    fn to_string(&self) -> String {
        match self {
            Origin::Web => "Web".to_string(),
            Origin::EDI => "EDI".to_string(),
            Origin::Unknown => "Unknown".to_string(),
        }
    }
}

impl Origin {
    pub fn from_optional_string(s: Option<String>) -> Self {
        match s {
            Some(s) => match s.as_str() {
                "Web" => Origin::Web,
                "EDI" => Origin::EDI,
                _ => Origin::Unknown,
            },
            None => Origin::Unknown,
        }
    }
}

// Order entity
#[derive(Debug, Clone)]
pub struct Order {
    c_order_id: u32,
    c_bpartner_id: u32,
    client_name: Option<String>,
    date: NaiveDate,
    order_ref: String,
    po_ref: Option<String>,
    origin: Origin,
    completion: Option<u32>,
    order_status: Option<String>,
    delivery_status: Option<String>,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.c_order_id == other.c_order_id
            && self.c_bpartner_id == other.c_bpartner_id
            && self.client_name == other.client_name
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
        client_name: Option<String>,
        date: NaiveDate,
        order_ref: String,
        po_ref: Option<String>,
        origin: Origin,
        completion: Option<u32>,
        order_status: Option<String>,
        delivery_status: Option<String>,
    ) -> Result<Self, DomainError> {
        // Validation is performed here
        // if completion.is_some() {
        //     Self::validate_completion(completion.unwrap())?;
        // }

        Ok(Self {
            c_order_id,
            c_bpartner_id,
            client_name,
            date,
            order_ref,
            po_ref,
            origin,
            completion,
            order_status,
            delivery_status,
        })
    }

    // TODO: bad implementation, use factory instead
    pub fn new_from_sting_dto(
        dto: OrderEntityFromStringDTO,
        date_format: &str,
    ) -> Result<Self, DomainError> {
        let c_order_id = dto.c_order_id.parse::<u32>().map_err(|err| {
            DomainError::ParsingError(
                err.to_string() + format!(": c_order_id => {}", dto.c_order_id).as_str(),
            )
        })?;
        let c_bpartner_id = dto.c_bpartner_id.parse::<u32>().map_err(|err| {
            DomainError::ParsingError(
                err.to_string() + format!(": c_bpartner_id => {}", dto.c_bpartner_id).as_str(),
            )
        })?;
        let date = NaiveDate::parse_from_str(dto.date.as_str(), date_format).map_err(|err| {
            DomainError::ParsingError(err.to_string() + format!(": date => {}", dto.date).as_str())
        })?;

        let completion = convert_string_to_option_string(dto.completion)
            .and_then(|s| {
                Some(
                    s.replace("%", "")
                        .parse::<f32>()
                        .map_err(|err| {
                            DomainError::ParsingError(
                                err.to_string() + format!(": completion => {}", s).as_str(),
                            )
                        })
                        .and_then(|number| Ok(number.round() as u32)),
                )
            })
            .transpose()?;
        let client_name = convert_string_to_option_string(dto.client_name);
        let po_ref = convert_string_to_option_string(dto.po_ref);
        let order_status = convert_string_to_option_string(dto.order_status);
        let delivery_status = convert_string_to_option_string(dto.delivery_status);

        let origin = match dto.origin.as_str() {
            "Web" => Origin::Web,
            "EDI" => Origin::EDI,
            _ => Origin::Unknown,
        };

        Self::new(
            c_order_id,
            c_bpartner_id,
            client_name,
            date,
            dto.order_ref,
            po_ref,
            origin,
            completion,
            order_status,
            delivery_status,
        )
    }

    // fn validate_completion(completion: u32) -> Result<(), DomainError> {
    //     if completion > 100 {
    //         Err(DomainError::ValidationError(format!(
    //             "Completion must be an integer between 0 and 100. {} given.",
    //             completion,
    //         )))
    //     } else {
    //         Ok(())
    //     }
    // }

    // Getters
    pub fn c_order_id(&self) -> u32 {
        self.c_order_id
    }

    pub fn c_bpartner_id(&self) -> u32 {
        self.c_bpartner_id
    }

    pub fn client_name(&self) -> Option<&str> {
        self.client_name.as_deref()
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

    pub fn origin(&self) -> &Origin {
        &self.origin
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

pub struct OrderEntityFromStringDTO {
    pub c_order_id: String,
    pub c_bpartner_id: String,
    pub client_name: String,
    pub date: String,
    pub order_ref: String,
    pub po_ref: String,
    pub origin: String,
    pub completion: String,
    pub order_status: String,
    pub delivery_status: String,
}

pub struct OrderDomainFactory {
    pub c_order_id: u32,
    pub c_bpartner_id: u32,
    pub client_name: Option<String>,
    pub date: NaiveDate,
    pub order_ref: String,
    pub po_ref: Option<String>,
    pub origin: Origin,
    pub completion: Option<u32>,
    pub order_status: Option<String>,
    pub delivery_status: Option<String>,
}

impl OrderDomainFactory {
    pub fn make(self) -> Result<Order, DomainError> {
        Order::new(
            self.c_order_id,
            self.c_bpartner_id,
            self.client_name,
            self.date,
            self.order_ref,
            self.po_ref,
            self.origin,
            self.completion,
            self.order_status,
            self.delivery_status,
        )
    }
}
