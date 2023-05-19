use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]

pub struct UsersCreation {
    pub id: String,
}
