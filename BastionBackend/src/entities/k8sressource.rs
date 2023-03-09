use diesel::{Queryable, AsChangeset, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::k8sressource;

#[derive(Queryable; Serialize)]
pub struct K8sRessource{
    pub name: string,
    pub ip_cluster: string,

}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name="k8sressource"]
pub struct K8sRessourceInsertable{
    pub name: string,
    pub ip_cluster: string,
}