use super::{DomainEntity, DomainError};

#[derive(Debug, PartialEq)]
pub struct MappingClient {
    id_customer: u32,
    idp_id_client: u32,
}

impl MappingClient {
    pub fn new(id_customer: u32, idp_id_client: u32) -> Result<Self, DomainError> {
        Ok(Self {
            id_customer,
            idp_id_client,
        })
    }

    pub fn from_i32(id_customer: i32, idp_id_client: i32) -> Result<Self, DomainError> {
        Ok(Self {
            id_customer: id_customer as u32,
            idp_id_client: idp_id_client as u32,
        })
    }

    pub fn id_customer(&self) -> u32 {
        self.id_customer
    }

    pub fn idp_id_client(&self) -> u32 {
        self.idp_id_client
    }
}

impl DomainEntity for MappingClient {}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub fn mapping_client_fixture() -> [MappingClient; 2] {
        [
            MappingClient::new(1, 1).unwrap(),
            MappingClient::new(2, 2).unwrap(),
        ]
    }
}
