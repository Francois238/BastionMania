use crate::schema::wireguardressource;
use serde::{Deserialize, Serialize};
use diesel::Insertable;


#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "wireguardressource"]
pub struct WireguardRessourceCreation {
    pub subnet_cidr: String,

}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "wireguardressource"]
pub struct WireguardRessourceSuppression {
    pub id: i32,
    pub id_bastion: String,
}