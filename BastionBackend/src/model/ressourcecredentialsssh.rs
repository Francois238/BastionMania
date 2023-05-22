use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct RessourceCredentialsSsh {
    pub pubkey: String,  
    pub username: String,
}