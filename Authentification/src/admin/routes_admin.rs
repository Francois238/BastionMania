use std::env;

use crate::admin::*;
use crate::api_error::ApiError;

use actix_web::{ post,patch,delete, web,  HttpResponse};
use jsonwebtoken::{ encode, Header, EncodingKey};
use actix_session::{Session};

//Pour s'enregistrer en tant qu'admin


#[post("/login/admin")]
pub async fn sign_in(session: Session, credentials: web::Json<AdminAuthentication>) -> Result<HttpResponse, ApiError> {

    let credentials = credentials.into_inner();

    //Verifie si le mail existe

    let admin = Admin::find_by_mail(credentials.mail)
    .map_err(|e| {
        match e.status_code {
            404 => ApiError::new(401, "Credentials not valid!".to_string()),
            _ => e,
        }
    })?;

    //Verifie si le password est ok

    let is_valid = admin.verify_password(credentials.password.as_bytes())?;


    if is_valid == true {

        let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

        let admin = AdminEnvoye::from_admin(admin); //Convertion vers la bonne structure

        let my_claims = Claims::from_admin(&admin, false); //Creation du corps du token, false car c est la 1ere etape de la 2FA

        let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref())).unwrap(); //Creation du jwt

        session.insert("claim", token).unwrap();

        Ok(HttpResponse::Ok().json(admin))
    }
    else {

        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }


}

#[post("/login/admin/otp")]
async fn double_authentication(session: Session, credentials: web::Json<CodeOtp>) -> Result<HttpResponse, ApiError>{
    

    let claims = verifier_session_2fa(&session).ok_or(ApiError::new(403, "Interdit".to_string())).map_err(|e| e)?; //verifie legitimite admin et que 2fa activee

    let cred = credentials.into_inner();

    let admin = Admin::verification_2fa(claims.mail.clone(), cred.code)?; //verification du code envoye par l admin pour le 2FA

    let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

    let my_claims = Claims::from_admin(&admin, true); //Creation du corps du token, true car 2FA etablie

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref())).unwrap(); //Creation du jwt

    session.insert("claim", token).unwrap();


    Ok(HttpResponse::Ok().json(admin))
    

}

#[post("/admins")]
async fn create_admin(admin: web::Json<AdminRecu>) -> Result<HttpResponse, ApiError> { //Enregistre un admin

    let admin = admin.into_inner();

    let _claims = verifier_session(&admin.claim).ok_or(ApiError::new(403, "Interdit".to_string())).map_err(|e| e)?;

    let admin = Admin::create(admin)?;
    Ok(HttpResponse::Ok().json(admin))


}

#[patch("/admins/{id}")]
async fn patch_admin( id: web::Path<i32>, cred: web::Json<AdminChangeCred> ) -> Result<HttpResponse, ApiError> { //Un admin peut modifier ses creds

    let cred = cred.into_inner();

    let id = id.into_inner();

    let claims = verifier_session(&cred.claim).ok_or(ApiError::new(403, "Interdit".to_string())).map_err(|e| e)?; //verifie legitimite admin
   
    if claims.id == id { //c'est bien l'admin lui meme qui veut changer ses creds

        Admin::update_password(id, cred).map_err(|_| ApiError::new(400, "Mauvaise requete".to_string()))?;

        Ok(HttpResponse::Ok().finish())


    }

    else{

        Err(ApiError::new(403, "Interdit".to_string()))
    }


}

#[post("/admins/{id}/otp")]
async fn create_otp_admin( id: web::Path<i32>, cred: web::Json<AdminChangeCred> ) -> Result<HttpResponse, ApiError> { //Un admin peut ajouter la 2FA a son compte

    let cred = cred.into_inner();

    let id = id.into_inner();

    let claims = verifier_session_activer_2fa(&cred.claim).ok_or(ApiError::new(403, "Interdit".to_string())).map_err(|e| e)?; //verifie legitimite admin
   
    if claims.id == id { //c'est bien l'admin lui meme qui veut activer la mfa

        Admin::create_otp(id, cred.password).map_err(|_| ApiError::new(400, "Mauvaise requete".to_string()))?;

        Ok(HttpResponse::Ok().finish())


    }

    else{

        Err(ApiError::new(403, "Interdit".to_string()))
    }


}


#[delete("/admins/{id}")]
async fn delete_admin( id: web::Path<i32>, cred: web::Json<AdminSupprimer>, ) -> Result<HttpResponse, ApiError> { //Un admin peut modifier ses creds

    let cred = cred.into_inner();

    let id = id.into_inner();

    let _claims = verifier_session(&cred.claim).ok_or(ApiError::new(403, "Interdit".to_string())).map_err(|e| e)?; //verifie legitimite admin

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