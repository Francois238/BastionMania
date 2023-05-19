use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BastionInstanceCreate {
    pub token: String,
    pub private_key: String,
    pub cidr_protege: String,
    pub agent_public_key: String,
    pub agent_endpoint: String,
    pub net_id: i32,
    pub bastion_port: i32,
}
