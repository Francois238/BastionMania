use reqwest::Response;
use std::{collections::HashMap, env};
use uuid::Uuid;

use crate::tools::{ApiError, Claims};

use super::AdminReceived;

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

pub async fn send_admin_to_authentication(
    admin: &AdminReceived,
    claims: Claims,
) -> Result<(), ApiError> {
    let url = env::var("AUTHENTICATION_URL")
        .map_err(|_| ApiError::new(500, "URL authentication missing".to_string()))?
        + "admins";

    let jwt = Claims::create_jwt(&claims)?;

    //creation du JSON a poster vers le micro service authentification
    let mut map = HashMap::new();
    map.insert("name".to_string(), admin.name.clone());
    map.insert("last_name".to_string(), admin.last_name.clone());
    map.insert("mail".to_string(), admin.mail.clone());
    map.insert("password".to_string(), admin.password.clone());
    map.insert("claim".to_string(), jwt);

    let client = reqwest::Client::new(); //Envoie une requete au micro service authentification pour ajouter l admin dans sa BDD
    let response = client.post(url).json(&map).send().await.map_err(|_| {
        ApiError::new(
            500,
            "Impossible to communicate with authentication".to_string(),
        )
    })?;

    response_error(response)?;

    Ok(())
}

pub async fn send_otp_to_authentication(
    secret_otp: String,
    claims: Claims,
) -> Result<(), ApiError> {
    let url = env::var("AUTHENTICATION_URL")
        .map_err(|_| ApiError::new(500, "URL authentication missing".to_string()))?
        + "admins/"
        + &claims.id.to_string()
        + "/otp";

    let jwt = Claims::create_jwt(&claims)?;

    //json a poster vers le micro service authentification
    let mut map = HashMap::new();
    map.insert("password".to_string(), secret_otp);
    map.insert("claim".to_string(), jwt);

    let client = reqwest::Client::new(); //Envoie une requete au micro service authentification pour ajouter la graine otp
    let response = client
        .post(url)
        .json(&map)
        .send()
        .await
        .map_err(|_| ApiError::new(500, "Internal server error".to_string()))?;

    response_error(response)?;

    Ok(())
}

pub async fn send_password_to_authentication(
    password: String,
    claims: &Claims,
) -> Result<(), ApiError> {
    let url = env::var("AUTHENTICATION_URL")
        .map_err(|_| ApiError::new(500, "URL authentication missing".to_string()))?
        + "admins/"
        + &claims.id.to_string();

    let jwt = Claims::create_jwt(claims)?;

    let mut map = HashMap::new();
    map.insert("password".to_string(), password);
    map.insert("claim".to_string(), jwt);

    let client = reqwest::Client::new(); //Envoie une requete au micro service authentification pour modifier le mot de passe

    let response = client
        .patch(url)
        .json(&map)
        .send()
        .await
        .map_err(|_| ApiError::new(500, "Internal server error".to_string()))?;

    response_error(response)?;

    Ok(())
}

pub async fn delete_admin_to_authentication(id: Uuid, claims: Claims) -> Result<(), ApiError> {
    let url = env::var("AUTHENTICATION_URL")
        .map_err(|_| ApiError::new(500, "URL authentication missing".to_string()))?
        + "admins/"
        + &id.to_string();

    let jwt = Claims::create_jwt(&claims)?;

    //creation du JSON a poster vers le micro service authentification
    let mut map = HashMap::new();
    map.insert("claim".to_string(), jwt);

    let client = reqwest::Client::new(); //Envoie une requete au micro service authentification pour supprimer l'admin dans sa BDD

    let response = client
        .delete(url)
        .json(&map)
        .send()
        .await
        .map_err(|_| ApiError::new(500, "Internal server error".to_string()))?;

    response_error(response)?;

    Ok(())
}

pub async fn first_use_to_authentication(admin: &AdminReceived) -> Result<(), ApiError> {
    let url = env::var("AUTHENTICATION_URL")
        .map_err(|_| ApiError::new(500, "URL authentication missing".to_string()))?
        + "premiere_utilisation";

    //creation du JSON a poster vers le micro service authentification
    let mut map = HashMap::new();
    map.insert("name".to_string(), admin.name.clone());
    map.insert("last_name".to_string(), admin.last_name.clone());
    map.insert("mail".to_string(), admin.mail.clone());
    map.insert("password".to_string(), admin.password.clone());
    map.insert("claim".to_string(), " ".to_string());

    let client = reqwest::Client::new(); //Envoie une requete au micro service authentification pour supprimer l'admin dans sa BDD

    let response = client
        .post(url)
        .json(&map)
        .send()
        .await
        .map_err(|_| ApiError::new(500, "Internal server error".to_string()))?;

    response_error(response)?;

    Ok(())
}
