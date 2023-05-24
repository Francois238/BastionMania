use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct RessourceCredentialsWireguard {
    pub pubkey: String,  
    pub user_net_id: i32,
}
#[derive(Serialize, Deserialize)]
pub struct ConfigWireguardInstanceCreate {
    pub uuid_user: String,
    pub uuid_ressource: String,
    pub pubkey: String,
    pub user_net_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ActivationWireguardSession{
    pub uuid_user: String,
    pub uuid_ressource: String,
    pub pubkey: String,
    pub ip: String,
    pub subnet_cidr: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DesactivationWireguardSession{
    pub uuid_user: String,
    pub uuid_ressource: String,
}

