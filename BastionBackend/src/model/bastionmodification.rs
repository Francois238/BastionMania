use crate::schema::bastion;
use diesel::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "bastion"]
pub struct BastionModification {
    pub name: String,
    pub subnet_cidr: String,
    pub agent_endpoint: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "bastion"]
pub struct BastionSuppression {
    pub id: i32,
}
