use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct BastionModification {
    pub bastion_name: String,
    pub subnet_cidr: String,

}

#[derive(Serialize, Deserialize)]
pub struct BastionSuppression {
    pub id: String,
}
