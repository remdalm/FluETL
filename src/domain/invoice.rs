use chrono::NaiveDate;
use rust_decimal::Decimal;

use super::{
    dto::date_dto::DateDTO,
    language::Language,
    vo::{
        file_name::FileName,
        locale::Locale,
        localized_item::{LocalizedItem, LocalizedItemFactory},
        price::Price,
        Reference, Translation,
    },
    DomainEntity, DomainError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Invoice {
    invoice_id: u32,
    client_id: u32,
    client_name: Option<String>,
    invoice_ref: Reference,
    file_name: Option<FileName>,
    date: NaiveDate,
    po_ref: Option<String>,
    invoice_types: Vec<LocalizedItem>,
    total_tax_excl: Price,
    total_tax_incl: Price,
}

impl DomainEntity for Invoice {}

impl Invoice {
    pub fn invoice_id(&self) -> u32 {
        self.invoice_id
    }

    pub fn client_id(&self) -> u32 {
        self.client_id
    }

    pub fn client_name(&self) -> Option<&str> {
        self.client_name.as_deref()
    }

    pub fn invoice_ref(&self) -> &str {
        self.invoice_ref.as_str()
    }

    pub fn file_name(&self) -> Option<String> {
        self.file_name.as_ref().map(|file| file.to_string())
    }

    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    pub fn po_ref(&self) -> Option<&str> {
        self.po_ref.as_deref()
    }

    pub fn invoice_types(&self) -> &[LocalizedItem] {
        &self.invoice_types
    }

    pub fn total_tax_excl(&self) -> Decimal {
        self.total_tax_excl.get_amount_as_decimal()
    }

    pub fn total_tax_incl(&self) -> Decimal {
        self.total_tax_incl.get_amount_as_decimal()
    }
}

pub struct InvoiceDomainFactory {
    pub invoice_id: u32,
    pub client_id: u32,
    pub client_name: Option<String>,
    pub invoice_ref: String,
    pub file_name: Option<String>,
    pub date_dto: DateDTO,
    pub po_ref: Option<String>,
    pub invoice_types: Vec<LocalizedItem>,
    pub total_tax_excl: String,
    pub total_tax_incl: String,
}

impl InvoiceDomainFactory {
    pub fn make(self) -> Result<Invoice, DomainError> {
        Ok(Invoice {
            invoice_id: self.invoice_id,
            client_id: self.client_id,
            client_name: self.client_name,
            invoice_ref: Reference::new(self.invoice_ref)?,
            file_name: self.file_name.map(FileName::try_from).transpose()?,
            date: self.date_dto.unwrap()?,
            po_ref: self.po_ref,
            invoice_types: self.invoice_types,
            total_tax_excl: Price::try_from(self.total_tax_excl)?,
            total_tax_incl: Price::try_from(self.total_tax_incl)?,
        })
    }
}

#[derive(Debug)]
pub struct InvoiceLocalizedTypeFactory {
    pub invoice_id: u32,
    pub locale: Locale,
    pub type_name: Translation,
}

impl LocalizedItemFactory for InvoiceLocalizedTypeFactory {
    fn make_from_language(&self, language: &Language) -> Result<LocalizedItem, DomainError> {
        if language.locale() != &self.locale {
            return Err(DomainError::ValidationError(format!(
                "Language locale {} does not match item locale {}",
                language.locale().as_str(),
                self.locale.as_str()
            )));
        }
        Ok(LocalizedItem::new(language.clone(), self.type_name.clone()))
    }

    fn get_language_locale(&self) -> &Locale {
        &self.locale
    }

    fn get_entity_id(&self) -> u32 {
        self.invoice_id
    }
}

#[cfg(test)]
pub mod tests {
    use crate::domain::vo::localized_item::tests::localized_item_fixtures;

    use super::*;
    pub fn invoice_fixtures() -> [Invoice; 2] {
        [
            Invoice {
                invoice_id: 1,
                client_id: 1,
                client_name: Some("Client 1".to_string()),
                invoice_ref: Reference::new("INV-1".to_string()).unwrap(),
                file_name: Some(FileName::try_from("INV-1.pdf".to_string()).unwrap()),
                date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                po_ref: Some("PO-1".to_string()),
                invoice_types: vec![
                    localized_item_fixtures()[0].clone(),
                    localized_item_fixtures()[1].clone(),
                ],
                total_tax_excl: Price::try_from("100.0".to_string()).unwrap(),
                total_tax_incl: Price::try_from("120.00".to_string()).unwrap(),
            },
            Invoice {
                invoice_id: 3,
                client_id: 1,
                client_name: Some("Client 1".to_string()),
                invoice_ref: Reference::new("INV-3".to_string()).unwrap(),
                file_name: Some(FileName::try_from("INV-3.pdf".to_string()).unwrap()),
                date: NaiveDate::from_ymd_opt(2020, 1, 3).unwrap(),
                po_ref: None,
                invoice_types: vec![localized_item_fixtures()[2].clone()],
                total_tax_excl: Price::try_from("-300.0".to_string()).unwrap(),
                total_tax_incl: Price::try_from("360.0".to_string()).unwrap(),
            },
        ]
    }
}
