use crate::schema::sshressource;
use serde::{Deserialize, Serialize};
use diesel::Insertable;


#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "sshressource"]
pub struct SshRessourceCreation {
    pub ip_machine: String,

}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "sshressource"]
pub struct SshRessourceSuppression {
    pub id: i32,
    pub id_bastion: String,
}