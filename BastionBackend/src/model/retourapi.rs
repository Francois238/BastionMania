use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RetourAPI<A> {
    pub success: bool,
    pub message: String,
    pub data: A,

}

