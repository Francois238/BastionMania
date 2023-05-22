use crate::schema::bastion_token;
use serde::{Deserialize, Serialize};
use diesel::Insertable;


#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "bastion_token"]
pub struct AgentProof {
    pub token: String,

}
