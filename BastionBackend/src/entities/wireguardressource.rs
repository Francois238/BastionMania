use crate::schema::wireguardressource;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct WireguardRessource {
    pub id: i32,
    pub id_bastion: i32,
    pub name: String,
    pub target_cidr: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "wireguardressource"]
pub struct WireguardRessourceInsertable {
    pub id: i32,
    pub id_bastion: i32,
    pub name: String,
    pub subnet_cidr: String,
}
