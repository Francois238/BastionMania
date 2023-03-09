use diesel::{Queryable, AsChangeset, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::sshressource;

#[derive(Queryable; Serialize)]
pub struct SshRessource{
    pub name: string,
    pub ip_machine: string,

}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name="sshressource"]
pub struct SshRessourceInsertable{
    pub name: string,
    pub ip_machine: string,
}