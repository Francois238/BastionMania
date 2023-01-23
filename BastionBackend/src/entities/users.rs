use diesel::{Queryable, AsChangeset, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Queryable)]
#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name="users"]
pub struct Users{
    pub id: i32,
    pub user_id: i32,
    pub bastion_id: i32,
    pub wireguard: bool,
    pub net_id: i32,

}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="users"]
pub struct UsersModification {
    pub user_id: i32,
    pub bastion_id: i32,
    pub wireguard: bool,
    pub net_id: i32,

}

