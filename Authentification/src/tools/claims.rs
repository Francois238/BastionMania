use crate::admin::AdminEnvoye;
use crate::tools::api_error::ApiError;
use crate::user::UserEnvoye;
use actix_web::HttpRequest;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Claims {
    //Struture composant le JWT
    pub id: Uuid,
    pub mail: String,
    pub admin: bool,
    pub otp: Option<bool>,             //Si l'otp est actif
    pub complete_authentication: bool, //Si par keycloack forcement ok sinon verifier mfa + changement mdp
    #[serde(with = "jwt_numeric_date")]
    pub iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    pub exp: OffsetDateTime,
}

pub struct Hours {
    pub iat: OffsetDateTime,
    pub exp: OffsetDateTime,
}

impl Hours {
    pub fn new() -> Hours {
        let iat1 = OffsetDateTime::now_utc();
        let exp1 = iat1 + Duration::hours(10);

        let iat = iat1
            .date()
            .with_hms_milli(iat1.hour(), iat1.minute(), iat1.second(), 0)
            .unwrap()
            .assume_utc();

        let exp = exp1
            .date()
            .with_hms_milli(exp1.hour(), exp1.minute(), exp1.second(), 0)
            .unwrap()
            .assume_utc();

        Hours { iat, exp }
    }
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

    pub fn new_user(user: &UserEnvoye, otp: Option<bool>, verif: bool) -> Claims {
        //Creation du JWT a partir des infos recuperees en BDD

        let iat = Hours::new().iat;
        let exp = Hours::new().exp;

        Claims {
            id: user.id,
            mail: user.mail.clone(),
            admin: false,
            otp,
            complete_authentication: verif,
            iat,
            exp,
        }
    }

    pub fn new_admin(admin: &AdminEnvoye, otp: Option<bool>, verif: bool) -> Claims {
        //Creation du JWT a partir des infos recuperees en BDD

        let iat = Hours::new().iat;
        let exp = Hours::new().exp;

        Claims {
            id: admin.id,
            mail: admin.mail.clone(),
            admin: true,
            otp,
            complete_authentication: verif,
            iat,
            exp,
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

    pub fn verify_admin_session_first(req: HttpRequest) -> Result<Claims, ApiError> {
        //Fct pour verifier valider du JWT 1ere etape connexion

        let jwt = Self::extract_jwt_header(req)?;

        let secret = Self::get_jwt_key()?;

        let token_message = decode::<Claims>(
            jwt.as_str(),
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        if token_message.claims.admin {
            //Si c est un admin et que la methode d authentification est classique
            return Ok(token_message.claims);
        }

        Err(ApiError::new(403, "Unauthorized".to_string()))
    }

    pub fn verify_user_session_first(req: HttpRequest) -> Result<Claims, ApiError> {
        let jwt = Self::extract_jwt_header(req)?;

        let secret = Self::get_jwt_key()?;

        let token_message = decode::<Claims>(
            jwt.as_str(),
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        if !token_message.claims.admin {
            //Si c est un user et que la methode d authentification est classique
            return Ok(token_message.claims);
        }

        Err(ApiError::new(403, "Unauthorized".to_string()))
    }

    pub fn verify_admin_session_ext(jwt: &str) -> Result<Claims, ApiError> {
        //Fct pour verifier valider du JWT 1ere etape connexion

        let secret = Self::get_jwt_key()?;

        let token_message = decode::<Claims>(
            jwt,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        if token_message.claims.admin {
            //Si c est un admin et que la methode d authentification est classique
            return Ok(token_message.claims);
        }

        Err(ApiError::new(403, "Unauthorized".to_string()))
    }

    pub fn verify_user_session_ext(jwt: &str) -> Result<Claims, ApiError> {
        let secret = Self::get_jwt_key()?;

        let token_message = decode::<Claims>(
            jwt,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        if !token_message.claims.admin {
            //Si c est un user et que la methode d authentification est classique
            return Ok(token_message.claims);
        }

        Err(ApiError::new(403, "Unauthorized".to_string()))
    }

    pub fn verify_admin_session_complete(jwt: &str) -> Result<Claims, ApiError> {
        let secret = Self::get_jwt_key()?;

        let token_message = decode::<Claims>(
            jwt,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        if token_message.claims.admin && token_message.claims.complete_authentication {
            //Si c est un admin et que il est completement authentifie
            return Ok(token_message.claims);
        }

        Err(ApiError::new(403, "Unauthorized".to_string()))
    }


    pub fn verify_user_session_complete(jwt: &str) -> Result<Claims, ApiError> {
        let secret = Self::get_jwt_key()?;

        let token_message = decode::<Claims>(
            jwt,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        if !token_message.claims.admin && token_message.claims.complete_authentication {
            //Si c est un admin et que il est completement authentifie
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
