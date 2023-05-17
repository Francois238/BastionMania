use crate::schema::k8sressource;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable; Serialize)]
pub struct K8sRessource {
    pub id: i32,
    pub id_bastion: i32,
    pub name: String,
    pub ip_cluster: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "k8sressource"]
pub struct K8sRessourceInsertable {
    pub id: i32,
    pub id_bastion: i32,
    pub name: String,
    pub ip_cluster: String,
}

