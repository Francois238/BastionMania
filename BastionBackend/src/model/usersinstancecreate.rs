use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct UsersInstanceCreate {
    pub user_id: String,
    pub bastion_id: String,
    pub wireguard: bool,
    pub user_net_id: i32,
}
