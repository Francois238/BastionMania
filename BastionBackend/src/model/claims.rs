use std::env;
use actix_web::HttpRequest;
use log::debug;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::api_error::ApiError;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    //Structure composant le JWT
    pub id: Uuid,
    pub mail: String,
    pub admin: bool,
    pub otp: Option<bool>,
    pub complete_authentication: bool,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
}

#[derive(Serialize, Debug)]
pub struct MyToken {
    pub jwt: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseToken{
    pub id: Uuid
}

mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}

pub fn extract_jwt_header(req: HttpRequest) -> Result<String, ApiError> {
    //Fct pour extraire le JWT du header

    let header = req.headers().get("Authorization");

    if header.is_none() {
        return Err(ApiError::new(404, "Unauthorized".to_string()));
    }

    let session = header.unwrap().to_str();

    if session.is_err() {
        return Err(ApiError::new(404, "Unauthorized".to_string()));
    }

    let jwt = session.unwrap().split("Bearer ").collect::<Vec<&str>>()[1];

    Ok(jwt.to_string())
}

pub async fn VerifyUser(req: HttpRequest) -> Result<Uuid, ApiError> {
    debug!("Verification user");

    let jwt = extract_jwt_header(req)?;

    let mytoken = MyToken {
        jwt
    };

    debug!("user token : {:?}", mytoken);

    let url = env::var("AUTHENTICATION_USER").map_err(|_| {
        ApiError::new(
            500,
            "Impossible to communicate with authentication".to_string(),
        )
    })?;

    let client = reqwest::Client::new(); //Envoie une requete au micro service user mangement pour ajouter le user dans sa BDD
    let response = client.get(url).json(&mytoken).send().await.map_err(|e| {
        ApiError::new(
            401,
            format!("error verification authorization: {}", e)
        )
    })?.json::<ResponseToken>().await.map_err(|e| {
        ApiError::new(
            401,
            format!("error verification authorization response : {}", e)
        )
    })?;

    let id = response.id;

    Ok(id)
}

pub async fn VerifyAdmin(req: HttpRequest) -> Result<Uuid, ApiError> {
    debug!("Verification admin");
    let jwt = extract_jwt_header(req)?;

    let token = MyToken {
        jwt
    };
    debug!("admin token : {:?}", mytoken);

    let url = env::var("AUTHENTICATION_ADMIN").map_err(|_| {
        ApiError::new(
            500,
            "Impossible to communicate with authentication".to_string(),
        )
    })?;

    let client = reqwest::Client::new(); //Envoie une requete au micro service user mangement pour ajouter le user dans sa BDD
    let response = client.get(url).json(&token).send().await.map_err(|e| {
        ApiError::new(
            401,
            format!("error verification authorization: {}", e)
        )
    })?.json::<ResponseToken>().await.map_err(|e| {
        ApiError::new(
            401,
            format!("error verification authorization response : {}", e)
        )
    })?;

    let id = response.id;

    Ok(id)
}


