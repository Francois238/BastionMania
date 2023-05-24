use crate::schema::user_config_wireguard;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize, Deserialize)]
pub struct UserConfigWireguard {
    pub id: i32,
    pub uuid_user: String,
    pub uuid_ressource: String,
    pub pubkey: String,
    pub user_net_id: i32,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable, Debug)]
#[table_name = "user_config_wireguard"]
pub struct UserConfigWireguardInsertable {
    pub uuid_user: String,
    pub uuid_ressource: String,
    pub pubkey: String,
    pub user_net_id: i32,
}