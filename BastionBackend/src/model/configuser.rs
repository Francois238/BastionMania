use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ConfigUser {
    pub id: String,
    pub net_id: i32,
}
