use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHRessource {
    pub name: String,
    pub ip: String,
    pub port: u16,
}