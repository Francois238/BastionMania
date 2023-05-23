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
    pub pubkey: String,
    pub user_net_id: i32,
}