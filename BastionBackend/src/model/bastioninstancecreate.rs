use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct BastionInstanceCreate {
    pub ssh_port: i32,
    pub wireguard_port: i32,
    pub bastion_id: String,
    pub net_id: i32,
}
