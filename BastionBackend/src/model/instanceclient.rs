use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceClient {
    pub public_key: String,
    pub allowed_ips: String,
}
