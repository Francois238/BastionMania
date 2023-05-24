use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct RessourceInstanceCreate {
    pub id: String,
    pub name: String,
    pub rtype: String,
    pub id_wireguard: Option<i32>,
    pub id_ssh: Option<i32>,   
    pub id_k8s: Option<i32>,    
}

#[derive(Serialize, Deserialize,Debug)]
pub struct RessourceSshInstanceCreate{
    pub id: String,
    pub name: String,
    pub ip_machine: String,
    pub port: i32,
    pub users: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RessourceWireguardInstanceCreate{
    pub id: String,
    pub user_id: String,
    pub pubkey: String,
    pub client_ip: String,
    pub target_ip: String,
}