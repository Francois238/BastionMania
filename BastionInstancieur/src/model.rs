use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BastionConfig {
    pub private_key: String,
    pub cidr_protege: String,
    pub agent_public_key: String,
    pub agent_endpoint: String,
    pub net_id: u8,
    pub bastion_port: u16,
}
