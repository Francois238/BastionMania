use crate::schema::user_config_wireguard;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize)]
pub struct UserConfigWireguard {
    pub id: i32,
    pub uuid_user: String,
    pub uuid_ressource: String,
    pub pubkey: String,
    pub user_net_id: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "user_config_wireguard"]
pub struct UserConfigWireguardInsertable {
    pub uuid_user: String,
    pub uuid_ressource: String,
    pub pubkey: String,
    pub user_net_id: i32,
}