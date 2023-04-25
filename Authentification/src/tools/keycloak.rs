use actix_web::HttpRequest;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

use super::ApiError;

#[derive(Serialize, Deserialize)]
pub struct Keycloak {
    pub email: String,
}

impl Keycloak {
    pub fn get_token(req: &HttpRequest) -> Result<String, ApiError> {
        //I want to read the header X-Userinfo decoded and return it as a string

        let header = req.headers().get("X-Userinfo");

        if header.is_none() {
            return Err(ApiError::new(403, "Unauthorized".to_string()));
        }

        let headerhttp = header.unwrap().to_str();

        let headerhttp = headerhttp.map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        let header = general_purpose::STANDARD
            .decode(headerhttp)
            .map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        let header = String::from_utf8(header)
            .map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        let admin = serde_json::from_str::<Keycloak>(&header);

        let admin = admin.map_err(|_| ApiError::new(403, "Unauthorized".to_string()))?;

        Ok(admin.email)
    }
}
