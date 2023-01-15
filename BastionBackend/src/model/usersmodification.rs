use diesel::Insertable;
use serde::{Deserialize, Serialize};
use crate::schema::users;


#[derive(Serialize, Deserialize)]

pub struct UsersCreation{
    pub id: i32,

}