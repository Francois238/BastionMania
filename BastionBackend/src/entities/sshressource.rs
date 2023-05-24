use crate::schema::sshressource;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize, Debug)]
pub struct SshRessource {
    pub id: i32,
    pub id_bastion: String,
    pub name: String,
    pub ip_machine: String,
    pub port: i32,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "sshressource"]
pub struct SshRessourceInsertable {
    pub id: i32,
    pub id_bastion: String,
    pub name: String,
    pub ip_machine: String,
    pub port: i32,
}
