use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct BastionModification {
    pub bastion_name: String,
    pub subnet_cidr: String,
    pub ssh_port: i32,
    pub wireguard_port: i32,
}

#[derive(Serialize, Deserialize)]
pub struct BastionSuppression {
    pub id: String,
}
