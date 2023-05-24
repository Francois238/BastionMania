use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigClient {
    pub client_private_key: String,
    pub client_address: String,
    pub bastion_public_key: String,
    pub bastion_endpoint: String,
    pub subnet_cidr: String,
}
