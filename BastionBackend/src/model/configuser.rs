use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigUser {
    pub id: i32,
    pub net_id: i32,
}
