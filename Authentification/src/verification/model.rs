use serde::{Deserialize};



#[derive(Deserialize)]
pub struct Token {
    //Token recu pour authentifier
    pub jwt: String,
}


