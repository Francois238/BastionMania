use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Deserialize)]
pub struct Token {
    //Token recu pour authentifier
    pub jwt: String,
}

#[derive(Deserialize, Serialize)]
pub struct Response{
    pub id: Uuid
}

