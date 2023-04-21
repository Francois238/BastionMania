use std::env;

use actix_web::HttpRequest;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use super::ApiError;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Claims {
    //Struture composant le JWT
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub mail: String,
    pub admin: bool,
    pub method: i32, //Methode d'authentification, 1 keycloack, 0 authenf classique
    pub otp: Option<bool>,
    pub complete_authentication: bool, //Si par keycloack forcement ok sinon verifier mfa + changement mdp
    #[serde(with = "jwt_numeric_date")]
    pub iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    pub exp: OffsetDateTime,
}

impl Claims {
    pub fn get_jwt_key() -> Result<String, ApiError> {
        //Fct pour recuperer la clef du JWT

        let secret =
            env::var("KEY_JWT").map_err(|_| ApiError::new(500, "KEY JWT missing".to_string()))?;

        Ok(secret)
    }

    pub fn create_jwt(claims: &Claims) -> Result<String, ApiError> {
        let secret_jwt = Claims::get_jwt_key()?;

        let jwt = encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(secret_jwt.as_ref()),
        )
        .map_err(|_| ApiError::new(500, "Impossible to create JWT".to_string()))?; //Creation du jwt

        Ok(jwt)
    }

    pub fn extract_jwt_header(req: HttpRequest) -> Result<String, ApiError> {
        //Fct pour extraire le JWT du header

        let header = req.headers().get("Authorization");

        if header.is_none() {
            return Err(ApiError::new(404, "Unauthorized".to_string()));
        }

        let session = header.unwrap().to_str();

        let jwt = session
            .map_err(|_| ApiError::new(404, "Unauthorized".to_string()))?
            .split("Bearer ")
            .collect::<Vec<&str>>()[1];

        Ok(jwt.to_string())
    }

    pub fn verify_admin_session_simple(req: HttpRequest) -> Result<Claims, ApiError> {
        //Fct pour verifier valider du JWT 1ere etape connexion

        let jwt = Self::extract_jwt_header(req)?;

        let secret = Self::get_jwt_key()?;

        let token_message = decode::<Claims>(
            jwt.as_str(),
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        if token_message.claims.admin && token_message.claims.method == 0 {
            //Si c est un admin et que la methode d authentification est classique
            return Ok(token_message.claims);
        }

        Err(ApiError::new(403, "Unauthorized".to_string()))
    }

    pub fn verify_admin_session_complete(req: HttpRequest) -> Result<Claims, ApiError> {
        //Fct pour verifier valider du JWT 1ere etape connexion

        let jwt = Self::extract_jwt_header(req)?;

        let secret = Self::get_jwt_key()?;

        let token_message = decode::<Claims>(
            jwt.as_str(),
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        if token_message.claims.admin && token_message.claims.complete_authentication {
            //Si c est un admin et que la methode d authentification est classique
            return Ok(token_message.claims);
        }

        Err(ApiError::new(403, "Unauthorized".to_string()))
    }
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
