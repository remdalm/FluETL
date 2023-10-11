use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CsvMappingClientDTO {
    pub c_bpartner_id: String,
    pub ad_user_id: String,
}

impl CsvDTO for CsvMappingClientDTO {}
