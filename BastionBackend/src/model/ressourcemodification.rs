use crate::schema::ressource;
use serde::{Deserialize, Serialize};
use diesel::Insertable;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "ressource"]
pub struct RessourceCreation {
    pub name: String,
    pub rtype: String,
 
}

#[derive(Serialize, Deserialize)]
pub struct RessourceSshCreation {
    pub name: String,
    pub rtype: String,
    pub ip_machine: String,
    pub port: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RessourceWireguardCreation {
    pub name: String,
    pub rtype: String,
    pub subnet_cidr: String,
}


#[derive(Serialize, Deserialize)]
pub struct RessourceSuppression {
    pub id: i32,
    pub id_bastion: String,
}