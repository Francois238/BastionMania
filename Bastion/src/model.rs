use serde::{Deserialize, Serialize};

pub struct WGToAgent{
    pub agent_endpoint: String,
    pub agent_public_key: String,
    pub private_key_path: String,
    pub net_cidr: String,
}

pub struct WGToClient{
    pub private_key_path: String,
    pub net_id: u8,
}

pub struct WGInterfaceConfig{
    pub listen_port: Option<u16>,
    pub private_key_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct WGPeerConfig{
    pub public_key: String,
    pub endpoint: Option<String>,
    pub allowed_ips: String,
}
