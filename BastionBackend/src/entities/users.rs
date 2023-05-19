use crate::schema::users;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct Users {
    pub id: i32,
    pub user_id: String,
    pub bastion_id: String,
    pub wireguard: bool,
    pub net_id: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct UsersModification {
    pub user_id: String,
    pub bastion_id: String,
    pub wireguard: bool,
    pub net_id: i32,
}
