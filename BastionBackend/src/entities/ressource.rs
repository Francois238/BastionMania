use diesel::{Queryable, AsChangeset, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::ressource;

#[derive(Queryable, Serialize)]
pub struct Ressource{
    pub id: string,
    pub id_bastion: string,
    pub name: string,
    pub _type: string,
    pub id_wireguard: string,
    pub id_ssh: string,
    pub id_k8s: string
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name="ressource"]
pub struct RessourceInsertable{
    pub name: string,
    pub _type: string,
    pub id_wireguard: string,
    pub id_ssh: string,
    pub id_k8s: string
}