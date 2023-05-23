use crate::schema::user_config_ssh;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize, Deserialize)]
pub struct UserConfigSsh {
    pub id: i32,
    pub uuid_user: String,
    pub uuid_ressource: String,
    pub pubkey: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "user_config_ssh"]
pub struct UserConfigSshInsertable {
    pub uuid_user: String,
    pub uuid_ressource: String,
    pub pubkey: String,
    pub username: String,
}