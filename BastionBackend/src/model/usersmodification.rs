use crate::schema::users;
use diesel::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct UsersCreation {
    pub id: i32,
}
