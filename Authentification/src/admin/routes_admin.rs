use std::env;

use crate::{admin::*, tools::password_management::verify_password};
use crate::tools::api_error::ApiError;

use actix_web::{post, patch, delete, web, HttpResponse, HttpRequest};
use jsonwebtoken::{ encode, Header, EncodingKey};
use uuid::Uuid;
use crate::tools::claims::Claims;

//Pour s'enregistrer en tant qu'admin


#[post("/login/admin")]
pub async fn sign_in(credentials: web::Json<AdminAuthentication>) -> Result<HttpResponse, ApiError> {

    let credentials = credentials.into_inner();

    //Verifie si le mail existe

    let admin = Admin::find_by_mail(credentials.mail)
    .map_err(|e| {
        match e.status_code {
            404 => ApiError::new(401, "Credentials not valid!".to_string()),
            _ => e,
        }
    })?;

   if admin.password.is_none() {  // l admin utilise keycloack
            return Err(ApiError::new(401, "Credentials not valid!".to_string()));
    }

    //Verifie si le password est ok

    let is_valid = verify_password(&admin.password.as_ref().unwrap(), credentials.password.as_bytes())?; //safe unwrap car on verifie si le password est non none avant


    if is_valid == true {

        let secret = env::var("KEY_JWT").map_err(|_| ApiError::new(500, format!("Failed to load key")))?;

        let admin = AdminEnvoye::from_admin(admin); //Convertion vers la bonne structure

        let my_claims = Claims::new_admin(&admin,0,Some(false), false); //Creation du corps du token ici authenf classique

        let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref())).map_err(|_| ApiError::new(500, format!("Failed to create jwt")))?;  //Creation du jwt

        let tok = "Bearer ".to_string() + &token;

        Ok(HttpResponse::Ok()
            .insert_header(("Authorization", tok))
            .insert_header(("Access-Control-Expose-Headers", "Authorization"))
            .json(admin)
        )
    }
    else {

        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }


}

#[post("/login/admin/otp")]
async fn double_authentication(req : HttpRequest, credentials: web::Json<CodeOtp>) -> Result<HttpResponse, ApiError>{
    

    let claims : Claims = Claims::verify_admin_session_first(req)?; //verifie legitimite admin et que 2fa activee

    let cred = credentials.into_inner();

    let admin = Admin::verification_2fa(claims.mail.clone(), cred.code)?; //verification du code envoye par l admin pour le 2FA

    let secret = env::var("KEY_JWT").map_err(|_| ApiError::new(500, format!("Failed to load key")))?;

    let change = admin.change; //recupere le changement de mdp

    let my_claims = Claims::new_admin(&admin,0,Some(true) , change.unwrap()); //Creation du corps du token, true car 2FA etablie

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref())).map_err(|_| ApiError::new(500, format!("Failed to create jwt")))?;  //Creation du jwt

    let tok = "Bearer ".to_string() + &token;

    Ok(HttpResponse::Ok()
        .insert_header(("Authorization", tok))
        .insert_header(("Access-Control-Expose-Headers", "Authorization"))
        .json(admin)
    )
    

}

#[post("/admins")]
async fn create_admin(admin: web::Json<AdminRecu>) -> Result<HttpResponse, ApiError> { //Enregistre un admin

    let admin = admin.into_inner();

    let _claims : Claims = Claims::verify_admin_session_complete(&admin.claim)?;

    let admin = Admin::create(admin)?;
    Ok(HttpResponse::Ok().json(admin))


}

#[patch("/admins/{id}")]
async fn patch_admin( id: web::Path<Uuid>, cred: web::Json<AdminChangeCred> ) -> Result<HttpResponse, ApiError> { //Un admin peut modifier ses creds

    let cred = cred.into_inner();

    let id = id.into_inner();

    let claims : Claims = Claims::verify_admin_session_ext(&cred.claim)?; //verifie legitimite admin
   
    if claims.id == id && claims.otp_active == Some(true) && claims.method==0{ //c'est bien l'admin lui meme qui veut changer ses creds

        Admin::update_password(id, cred).map_err(|_| ApiError::new(400, "Mauvaise requete".to_string()))?;

        Ok(HttpResponse::Ok().finish())


    }

    else{

        Err(ApiError::new(403, "Interdit".to_string()))
    }


}

#[post("/admins/{id}/otp")]
async fn create_otp_admin( id: web::Path<Uuid>, cred: web::Json<AdminChangeCred> ) -> Result<HttpResponse, ApiError> { //Un admin peut ajouter la 2FA a son compte

    let cred = cred.into_inner();

    let id = id.into_inner();

    let claims: Claims = Claims::verify_admin_session_complete(&cred.claim)?; //verifie legitimite admin
   
    if claims.id == id && claims.method==0 { //c'est bien l'admin lui meme qui veut activer la mfa

        Admin::create_otp(id, cred.password).map_err(|_| ApiError::new(400, "Mauvaise requete".to_string()))?;

        Ok(HttpResponse::Ok().finish())


    }

    else{

        Err(ApiError::new(403, "Interdit".to_string()))
    }


}


#[delete("/admins/{id}")]
async fn delete_admin( id: web::Path<Uuid>, cred: web::Json<AdminSupprimer>, ) -> Result<HttpResponse, ApiError> { //Un admin peut modifier ses creds

    let cred = cred.into_inner();

    let id = id.into_inner();

    let _claims : Claims = Claims::verify_admin_session_complete(&cred.claim)?; //verifie legitimite admin

    Admin::delete(id).map_err(|_| ApiError::new(400, "Mauvaise requete".to_string()))?;

    Ok(HttpResponse::Ok().finish())



}

#[post("/premiere_utilisation")]
async fn premiere_utilisation(admin: web::Json<AdminRecu>) -> Result<HttpResponse, ApiError> { //Enregistre un admin


    let admin = admin.into_inner();

    let admin = premiere_utilisation_bastion(admin)?;
    Ok(HttpResponse::Ok().json(admin))


}





pub fn routes_admin(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_in);
    cfg.service(create_admin);
    cfg.service(double_authentication);
    cfg.service(create_otp_admin);
    cfg.service(patch_admin);
    cfg.service(delete_admin);
    cfg.service(premiere_utilisation);
}