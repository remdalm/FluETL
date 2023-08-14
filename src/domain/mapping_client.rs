use super::{DomainEntity, DomainError};

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
    pub fn new_from_string(
        idp_id_client: String,
        ps_id_customer: String,
    ) -> Result<Self, DomainError> {
        let idp_id_client = idp_id_client
            .parse::<u32>()
            .map_err(|err| DomainError::ParsingError(err.to_string()))?;
        let ps_id_customer = ps_id_customer
            .parse::<u32>()
            .map_err(|err| DomainError::ParsingError(err.to_string()))?;

        Self::new(idp_id_client, ps_id_customer)
    }
}

impl DomainEntity for MappingClient {}
