use diesel::{Queryable, AsChangeset, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::wireguardressource;

#[derive(Queryable; Serialize)]
pub struct WireguardRessource{
    pub name: string,
    pub target_cidr: string,

}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name="wireguardressource"]
pub struct WireguardRessourceInsertable{
    pub name: string,
    pub target_cidr: string,
}