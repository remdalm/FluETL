use crate::domain::{DomainError, MappingClient};
use crate::infrastructure::csv_reader::CsvMappingClientDTO;

impl From<CsvMappingClientDTO> for Result<MappingClient, DomainError> {
    fn from(dto: CsvMappingClientDTO) -> Result<MappingClient, DomainError> {
        MappingClient::new_from_string(dto.c_bpartner_id, dto.ad_user_id)
    }
}
