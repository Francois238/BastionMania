use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct RessourceCredentialsWireguard {
    pub pubkey: String,  
    
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
    pub id: String,
    pub id_client: String,
    pub public_key: String,
    pub client_ip: String,
    pub target_ip: String,
}

#[derive(Serialize, Deserialize)]
pub struct DesactivationWireguardSession{
    pub uuid_user: String,
    pub uuid_ressource: String,
}

