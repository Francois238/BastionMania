use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigWireguardRetour{
    pub bastion_pubkey: String,
    pub user_ip: String,
    pub ressource_ip: String,
    pub port_wireguard: i32,
}