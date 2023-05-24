use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct RessourceCredentialsSsh {
    pub pubkey: String,  
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigSshInstanceCreate {
    pub uuid_user: String,
    pub uuid_ressource: String,
    pub pubkey: String,
    pub username: String,
}


#[derive(Serialize, Deserialize)]
pub struct ActivationSshSession{
    pub uuid_user: String,
    pub username: String,
    pub ip: String,
    pub port: i32,
    pub users: Vec<String>,
    
}

#[derive(Serialize, Deserialize)]
pub struct DesactivationSshSession{
    pub uuid_user: String,
    pub uuid_ressource: String,
    
}