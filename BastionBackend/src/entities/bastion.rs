use crate::schema::{bastion, bastion_token};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize, Debug)]

pub struct Bastion {
    pub bastion_id: String,
    pub name: String,
    pub subnet_cidr: String,
    pub ssh_port: i32,
    pub wireguard_port: i32,
    pub net_id: i32, // 10.10.x.y => c'est le x
}
#[derive(Serialize, Deserialize, AsChangeset, Insertable, Debug)]
#[table_name = "bastion"]
pub struct BastionInsertable {
    pub bastion_id: String,
    pub name: String,
    pub subnet_cidr: String,
    pub ssh_port: i32,
    pub wireguard_port: i32,
    pub net_id: i32, // 10.10.x.y => c'est le x
}

#[derive(Queryable, Serialize, Debug )]
pub struct BastionToken {
    pub bastion_id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Debug)]
#[table_name = "bastion_token"]
pub struct BastionTokenInsertable {
    pub bastion_id: String,
    pub token: String,
}
