use diesel::{Queryable, AsChangeset, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::bastion;

#[derive(Queryable)]
#[derive(Serialize)]

pub struct Bastion{
    pub id: i32,
    pub name: String,
    pub subnet_cidr: String,
    pub agent_endpoint: String,
    pub pubkey: String,
    pub port: i32,
    pub net_id: i32 // 10.10.x.y => c'est le x
}
#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name="bastion"]
pub struct BastionInsertable{
    pub name: String,
    pub subnet_cidr: String,
    pub agent_endpoint: String,
    pub pubkey: String,
    pub port: i32,
    pub net_id: i32 // 10.10.x.y => c'est le x
}
