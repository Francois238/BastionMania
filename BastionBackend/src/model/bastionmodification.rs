use diesel::Insertable;
use serde::{Deserialize, Serialize};
use crate::schema::bastion;

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="bastion"]
pub struct BastionModification {
    pub name: String,
    pub subnet_cidr: String,
    pub agent_endpoint: String,
}