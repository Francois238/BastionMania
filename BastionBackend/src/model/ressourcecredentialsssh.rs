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
    pub public_key: SSHPublicKey,
    
}
#[derive(Serialize, Deserialize)]
pub struct SSHPublicKey{
    pub algo: String,
    pub key: String,
}

impl SSHPublicKey{
    pub fn from_string(key: String) -> SSHPublicKey{
        let mut key = key.split(" ");
        let algo = key.next().unwrap().to_string();
        let key = key.next().unwrap().to_string();
        SSHPublicKey{
            algo,
            key,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DesactivationSshSession{
    pub uuid_user: String,
    pub uuid_ressource: String,
    
}