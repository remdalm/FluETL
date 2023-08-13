use super::DomainError;

#[derive(Debug, PartialEq)]
pub struct MappingClient {
    idp_id_client: u32,
    ps_id_customer: u32,
}

impl MappingClient {
    pub fn new(idp_id_client: u32, ps_id_customer: u32) -> Result<Self, DomainError> {
        Ok(Self {
            idp_id_client,
            ps_id_customer,
        })
    }
}
