use crate::schema::{bastion, bastion_token};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize)]

pub struct Bastion {
    pub bastion_id: String,
    pub name: String,
    pub subnet_cidr: String,
    pub agent_endpoint: String,
    pub pubkey: String,
    pub port: i32,
    pub net_id: i32, // 10.10.x.y => c'est le x
}
#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "bastion"]
pub struct BastionInsertable {
    pub bastion_id: String,
    pub name: String,
    pub subnet_cidr: String,
    pub agent_endpoint: String,
    pub pubkey: String,
    pub port: i32,
    pub net_id: i32, // 10.10.x.y => c'est le x
}

#[derive(Queryable, Serialize)]
pub struct BastionToken {
    pub bastion_id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "bastion_token"]
pub struct BastionTokenInsertable {
    pub bastion_id: String,
    pub token: String,
}
