use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct RessourceCredentialsSsh {
    pub pubkey: String,  
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigSshInstanceCreate {
    pub uuid_user: String,
    pub pubkey: String,
    pub username: String,
}


#[derive(Serialize, Deserialize)]
pub struct ActivationSshSession{
    pub id: String,
    pub name: String,
    pub public_key: String,
    
}

#[derive(Serialize, Deserialize)]
pub struct DesactivationSshSession{
    pub uuid_user: String,
    pub uuid_ressource: String,
    
}