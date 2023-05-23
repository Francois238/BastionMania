use std::env::VarError;
use serde::{Deserialize, Serialize};

pub struct LaunchConfig{
    pub bm_host: String,
    pub agent_host: String,
    pub token: String,
}

impl LaunchConfig {
    pub fn new() -> Result<Self, VarError>{
        Ok(Self {
            bm_host: std::env::var("BM_HOST")?,
            agent_host: std::env::var("AGENT_HOST")?,
            token: std::env::var("TOKEN")?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct PairConfig{
    pub token: String,
    pub public_key: String,
    pub agent_host: String,
}