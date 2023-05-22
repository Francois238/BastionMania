use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigAgent {
    pub pubkey: String,
    pub endpoint: String,
    pub target_cidr: String,
    pub token: String,
}
