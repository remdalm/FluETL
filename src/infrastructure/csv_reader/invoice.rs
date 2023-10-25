use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CsvInvoiceDTO {
    pub c_invoice_id: String,
    pub c_bpartner_id: String,
    pub client_name: String,
    pub invoice_ref: String,
    pub file_name: String,
    pub date: String,
    pub po_ref: String,
    pub total_tax_excl: String,
    pub total_tax_incl: String,
}

impl CsvDTO for CsvInvoiceDTO {}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CsvInvoiceLocalizedItemDTO {
    pub c_invoice_id: String,
    pub ad_language: String,
    pub invoice_type: String,
}

impl CsvDTO for CsvInvoiceLocalizedItemDTO {}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub fn csv_invoice_dto_fixtures() -> [CsvInvoiceDTO; 3] {
        [
            CsvInvoiceDTO {
                c_invoice_id: "1".to_string(),
                c_bpartner_id: "1".to_string(),
                client_name: "Client 1".to_string(),
                invoice_ref: "INV-1".to_string(),
                file_name: "INV-1.pdf".to_string(),
                date: "2020-01-01".to_string(),
                po_ref: "PO-1".to_string(),
                total_tax_excl: "100.00".to_string(),
                total_tax_incl: "120.0".to_string(),
            },
            CsvInvoiceDTO {
                c_invoice_id: "2".to_string(),
                c_bpartner_id: "2".to_string(),
                client_name: "Client 2".to_string(),
                invoice_ref: "INV-2".to_string(),
                file_name: "INV -2.pdf".to_string(), // Note the space in the file name
                date: "2020-01-02".to_string(),
                po_ref: "PO-2".to_string(),
                total_tax_excl: "200.0".to_string(),
                total_tax_incl: "240.00".to_string(),
            },
            CsvInvoiceDTO {
                c_invoice_id: "3".to_string(),
                c_bpartner_id: "1".to_string(),
                client_name: "Client 1".to_string(),
                invoice_ref: "INV-3".to_string(),
                file_name: "INV-3.pdf".to_string(),
                date: "2020-01-03".to_string(),
                po_ref: String::new(),
                total_tax_excl: "-300.0".to_string(),
                total_tax_incl: "360.00".to_string(),
            },
        ]
    }
}
