use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UsersInstanceCreate {
    pub user_id: i32,
    pub bastion_id: i32,
    pub wireguard: bool,
    pub user_net_id: i32,
}
