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