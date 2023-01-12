use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigAgent {
    pub privkey: String,
    pub pubkey: String,
}