use std::env;

use crate::user::*;
use crate::api_error::ApiError;

use actix_web::{ post, patch,delete, web,  HttpResponse};
use jsonwebtoken::{ encode, Header, EncodingKey};
use actix_session::{Session};

//Pour s'enregistrer en tant que user


#[post("/login")]
pub async fn sign_in(session: Session, credentials: web::Json<UserAuthentication>) -> Result<HttpResponse, ApiError> {

    let credentials = credentials.into_inner();

    //Verifie si le mail existe

    let user = User::find_by_mail(credentials.mail)
    .map_err(|e| {
        match e.status_code {
            404 => ApiError::new(401, "Credentials not valid!".to_string()),
            _ => e,
        }
    })?;

    //Verifie si le password est ok

    let is_valid = user.verify_password(credentials.password.as_bytes())?;


    if is_valid == true {

        let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

        let user = UserEnvoye::from_user(user); //Convertion vers la bonne structure

        let my_claims = Claims::from_user(&user, false); //Creation du corps du token

        let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref())).unwrap(); //Creation du jwt

        session.insert("claim", token).unwrap();

        Ok(HttpResponse::Ok().json(user))
    }
    else {

        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }


}

#[post("/login/otp")]
async fn double_authentication(session: Session, credentials: web::Json<CodeOtp>) -> Result<HttpResponse, ApiError>{
    

    let claims = verifier_session_2fa(&session).ok_or(ApiError::new(403, "Interdit".to_string())).map_err(|e| e)?; //verifie legitimite du user et que 2fa activee

    let cred = credentials.into_inner();

    let user = User::verification_2fa(claims.mail.clone(), cred.code)?; //verification du code envoye par le user pour le 2FA

    let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

    let my_claims = Claims::from_user(&user, true); //Creation du corps du token, true car 2FA etablie

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref())).unwrap(); //Creation du jwt

    session.insert("claim", token).unwrap();


    Ok(HttpResponse::Ok().json(user))
    

}

#[post("/users")]
async fn create_user(user: web::Json<UserRecu>) -> Result<HttpResponse, ApiError> { //Enregistre un user

    let user = user.into_inner();

    let _claims = verifier_session_admin(&user.claim).ok_or(ApiError::new(403, "Interdit".to_string())).map_err(|e| e)?;

    let user = User::create(user)?;
    Ok(HttpResponse::Ok().json(user))

 
}

#[patch("/users/{id}")]
async fn patch_user( id: web::Path<i32>, cred: web::Json<UserChangeCred>, ) -> Result<HttpResponse, ApiError> { //Un user peut modifier ses creds

    let cred = cred.into_inner();

    let id = id.into_inner();

    let claims = verifier_session_user(&cred.claim).ok_or(ApiError::new(403, "Interdit".to_string())).map_err(|e| e)?; //verifie legitimite du user
   
    if claims.id == id { //c'est bien le user lui meme qui veut changer ses creds

        User::update_password(id, cred).map_err(|_| ApiError::new(400, "Mauvaise requete".to_string()))?;
        Ok(HttpResponse::Ok().finish())


    }

    else{

        Err(ApiError::new(403, "Interdit".to_string()))
    }


}

#[post("/users/{id}/otp")]
async fn create_otp_user( id: web::Path<i32>, cred: web::Json<UserChangeCred> ) -> Result<HttpResponse, ApiError> { //Un user peut ajouter la 2FA a son compte

    let cred = cred.into_inner();

    let id = id.into_inner();

    let claims = verifier_session_activer_2fa(&cred.claim).ok_or(ApiError::new(403, "Interdit".to_string())).map_err(|e| e)?; //verifie legitimite admin
   
    if claims.id == id { //c'est bien l'admin lui meme qui veut activer la mfa

        User::create_otp(id, cred.password).map_err(|_| ApiError::new(400, "Mauvaise requete".to_string()))?;

        Ok(HttpResponse::Ok().finish())


    }

    else{

        Err(ApiError::new(403, "Interdit".to_string()))
    }


}

#[delete("/users/{id}")]
async fn delete_user( id: web::Path<i32>, cred: web::Json<UserSupprimer>, ) -> Result<HttpResponse, ApiError> { //Un admin peut modifier ses creds

    let cred = cred.into_inner();

    let id = id.into_inner();

    let _claims = verifier_session_admin(&cred.claim).ok_or(ApiError::new(403, "Interdit".to_string())).map_err(|e| e)?; //verifie legitimite admin

    User::delete(id).map_err(|_| ApiError::new(400, "Mauvaise requete".to_string()))?;

    Ok(HttpResponse::Ok().finish())



}





pub fn routes_user(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_in);
    cfg.service(double_authentication);
    cfg.service(create_user);
    cfg.service(patch_user);
    cfg.service(create_otp_user);
    cfg.service(delete_user);
}