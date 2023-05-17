use crate::schema::k8sressource;
use serde::{Deserialize, Serialize};
use diesel::Insertable;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "k8sressource"]
pub struct K8sRessourceCreation {
    pub ip_cluster: String,

}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "k8sressource"]
pub struct K8sRessourceSuppression {
    pub id: i32,
    pub id_bastion: i32,
}