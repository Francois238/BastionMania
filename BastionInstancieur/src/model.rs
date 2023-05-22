use kube::Client;
use serde::{Deserialize, Serialize};
use std::env::VarError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BastionConfig {
    pub ssh_port: u16,
    pub wireguard_port: u16,
    pub bastion_id: String,
    pub net_id: u8,
}

pub struct InstancieurConfig {
    pub image: String,
    pub client: Client,
}

impl InstancieurConfig {
    pub fn new(client: Client) -> Result<Self, VarError> {
        let image = std::env::var("BASTION_IMAGE")?;
        Ok(Self { image, client })
    }
}
