use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct AgentAskPairInfo {
    pub token: String,
    pub public_key: String,
    pub agent_host: String,
}

#[derive(Serialize, Deserialize)]
pub struct AgentPairInfo{
    pub agent_host: String,
    pub public_key: String,
    pub target_cidr: String,
}