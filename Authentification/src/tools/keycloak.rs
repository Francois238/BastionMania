use actix_web::{Responder, HttpResponse, web, HttpRequest};
use base64::{Engine as _, engine::{general_purpose}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Keycloak{
    pub preferred_username : String,
    pub email_verified : bool,
    pub username : String,
    pub id : String,
    pub email : String,
    pub sub : String,
    pub given_name : String,
    pub family_name : String,
}

impl Keycloak {

    fn get_token(req: HttpRequest) -> Result<String, ApiError> {

        //I want to read the header X-Userinfo decoded and return it as a string
    
        let header = req.headers().get("X-Userinfo");

        if header.is_none() {
            return Err(ApiError::new(403,"Unauthorized".to_string()));
        }
    
        let headerhttp = header.to_str().unwrap();
    
        let header =general_purpose::STANDARD.decode(headerhttp);

        if header.is_none() {
            return Err(ApiError::new(403,"Unauthorized".to_string()));
        }

        let header = String::from_utf8(header).unwrap();

        let admin = serde_json::from_str::<Keycloak>(&header);

        if admin.is_err() {
            return Err(ApiError::new(403,"Unauthorized".to_string()));
        }   

        let admin = admin.unwrap();

        return Ok(admin.mail)

    }

}

