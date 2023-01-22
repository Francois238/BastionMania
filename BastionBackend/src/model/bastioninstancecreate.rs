use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BastionInstanceCreate {
    pub privkey: String,
    pub subnet_cidr: String,
    pub agent_pubkey: String,
    pub agent_endpoint: String,
    pub net_id: i32,
}