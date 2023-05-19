use crate::schema::ressource;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Ressource {
    pub id: String,
    pub id_bastion: String,
    pub name: String,
    pub rtype: String,
    pub id_wireguard: Option<i32>,
    pub id_ssh: Option<i32>,
    pub id_k8s: Option<i32>,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "ressource"]
pub struct RessourceInsertable {
    pub id: String,
    pub id_bastion: String,
    pub name: String,
    pub rtype: String,
    pub id_wireguard: Option<i32>,
    pub id_ssh: Option<i32>,
    pub id_k8s: Option<i32>,
}
