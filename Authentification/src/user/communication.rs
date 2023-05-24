use actix_web::HttpResponse;
use reqwest::Response;
use std::env;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tools::{ApiError, Claims, Hours};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sent {
    //structure envoyee vers authentication
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub mail: String,
    pub claims: String,
}

impl Sent {
    pub fn new(id: Uuid, name: String, last_name: String, mail: String) -> Self {
        let iat = Hours::new().iat;
        let exp = Hours::new().exp;

        let claims = Claims {
            id,
            mail: mail.clone(),
            admin: false,
            otp: None,
            complete_authentication: false,
            iat,
            exp,
        };

        let jwt = Claims::create_jwt(&claims).unwrap();

        Self {
            id,
            name,
            last_name,
            mail,
            claims: jwt,
        }
    }

    pub async fn sent(user: Sent) -> Result<(), ApiError> {
        let url = env::var("URL_USER_MANAGEMENT").map_err(|_| {
            ApiError::new(
                500,
                "Impossible to communicate with authentication".to_string(),
            )
        })?;

        let client = reqwest::Client::new(); //Envoie une requete au micro service user mangement pour ajouter le user dans sa BDD
        let response = client.post(url).json(&user).send().await.map_err(|_| {
            ApiError::new(
                500,
                "Impossible to communicate with authentication".to_string(),
            )
        })?;

        response_error(response)?;

        Ok(())
    }
}

fn response_error(response: Response) -> Result<(), ApiError> {
    if response.status().is_server_error() {
        return Err(ApiError::new(
            500,
            "Internal authentication error".to_string(),
        ));
    }

    if response.status().is_client_error() {
        return Err(ApiError::new(400, "Bad request".to_string()));
    }

    Ok(())
}

pub fn redirection_err() ->HttpResponse {

    let redirection = env::var("REDIRECT_URL_USER").unwrap();
    
    HttpResponse::Found() // Ou HttpResponse::TemporaryRedirect() si vous souhaitez un code 307
        .append_header(("Location", redirection))
        .finish()
    
}
