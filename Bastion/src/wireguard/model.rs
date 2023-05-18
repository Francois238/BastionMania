use serde::{Deserialize, Serialize};

use super::wgconfigure;

pub struct WGToAgent {
    pub agent_endpoint: String,
    pub agent_public_key: String,
    pub private_key_path: String,
    pub net_cidr: String,
}

pub struct WGToClient {
    pub private_key_path: String,
    pub net_id: u8,
}

pub struct WGInterfaceConfig {
    pub listen_port: Option<u16>,
    pub private_key_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WGPeerConfig {
    pub public_key: String,
    pub endpoint: Option<String>,
    pub allowed_ips: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WGPeerPublicKey {
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WireguardRessource {
    pub id: String,
    pub client_id: String,
    pub public_key: String,
    pub client_ip: String,
    pub target_ip: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WireguardAgent {
    pub public_key: String,
    pub endpoint: String,
    pub target_cidr: String,
}

impl WireguardRessource {
    pub fn to_wg_peer_config(&self) -> WGPeerConfig {
        WGPeerConfig {
            public_key: self.public_key.clone(),
            endpoint: None,
            allowed_ips: self.client_ip.clone(),
        }
    }

    pub fn create(&self) -> Result<(), String> {
        wgconfigure::add_peer("wg-client", &self.to_wg_peer_config())?;
        wgconfigure::allow_target_ip(self)
    }

    pub fn delete(&self) -> Result<(), String> {
        wgconfigure::remove_peer("wg-client", &self.public_key)?;
        wgconfigure::deny_target_ip(self)
    }
}
