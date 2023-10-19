use chrono::NaiveDate;

use super::{
    dto::date_dto::DateDTO,
    vo::{completion::Completion, status::Status, Reference},
    DomainEntity, DomainError,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Origin {
    Web,
    Edi,
    Unknown,
}

impl ToString for Origin {
    fn to_string(&self) -> String {
        match self {
            Origin::Web => "Web".to_string(),
            Origin::Edi => "EDI".to_string(),
            Origin::Unknown => "Unknown".to_string(),
        }
    }
}

impl Origin {
    pub fn from_optional_string(s: Option<String>) -> Self {
        // Todo : use a regex
        match s {
            Some(s) => match s.as_str() {
                "Web" => Origin::Web,
                "EDI" => Origin::Edi,
                _ => Origin::Unknown,
            },
            None => Origin::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Order {
    order_id: u32,
    client_id: u32,
    client_name: Option<String>,
    date: NaiveDate,
    order_ref: Reference,
    po_ref: Option<String>,
    origin: Option<Origin>,
    completion: Option<Completion>,
    order_status: Option<Status>,
}

impl Order {
    // Getters
    pub fn order_id(&self) -> u32 {
        self.order_id
    }

    pub fn client_id(&self) -> u32 {
        self.client_id
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

    pub fn origin(&self) -> Option<&Origin> {
        self.origin.as_ref()
    }

    pub fn completion(&self) -> Option<u32> {
        self.completion.as_ref().map(|c| c.as_u32())
    }

    pub fn order_status(&self) -> Option<&Status> {
        self.order_status.as_ref()
    }
}

impl DomainEntity for Order {}

pub struct OrderDomainFactory {
    pub order_id: u32,
    pub client_id: u32,
    pub client_name: Option<String>,
    pub date_dto: DateDTO,
    pub order_ref: String,
    pub po_ref: Option<String>,
    pub origin: Option<String>,
    pub completion: Option<Completion>,
    pub order_status: Option<String>,
}

impl OrderDomainFactory {
    pub fn make(self) -> Result<Order, DomainError> {
        let origin = self.origin.map(|s| match s.as_str() {
            "Web" => Origin::Web,
            "EDI" => Origin::Edi,
            _ => Origin::Unknown,
        });

        Ok(Order {
            order_id: self.order_id,
            client_id: self.client_id,
            client_name: self.client_name,
            date: self.date_dto.unwrap()?,
            order_ref: Reference::new(self.order_ref)?,
            po_ref: self.po_ref,
            origin,
            completion: self.completion,
            order_status: self.order_status.map(|s| Status::from(s.as_str())),
        })
    }
}

pub mod tests {
    use super::*;
    pub fn order_fixtures() -> [Order; 3] {
        [
            Order {
                order_id: 1,
                client_id: 1,
                client_name: Some("Client 1".to_string()),
                order_ref: Reference::new("Ref1".to_string()).unwrap(),
                date: chrono::NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
                po_ref: Some("PoRef1".to_string()),
                origin: Some(Origin::Web),
                completion: Some(Completion::from(30)),
                order_status: Some(Status::Completed),
            },
            Order {
                order_id: 2,
                client_id: 2,
                client_name: Some("Client 2".to_string()),
                order_ref: Reference::new("Ref2".to_string()).unwrap(),
                date: chrono::NaiveDate::from_ymd_opt(2023, 8, 2).unwrap(),
                po_ref: Some("PoRef2".to_string()),
                origin: Some(Origin::Edi),
                completion: Some(Completion::from(20)),
                order_status: Some(Status::Invalid),
            },
            Order {
                order_id: 3,
                client_id: 1,
                client_name: None,
                order_ref: Reference::new("Ref3".to_string()).unwrap(),
                date: chrono::NaiveDate::from_ymd_opt(2023, 8, 3).unwrap(),
                po_ref: None,
                origin: None,
                completion: None,
                order_status: None,
            },
        ]
    }
}
