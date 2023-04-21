use crate::tools::api_error::ApiError;
use crate::tools::Keycloak;
use crate::user::*;

use crate::tools::claims::Claims;
use crate::tools::password_management::verify_password;
use actix_web::{delete, patch, post, web, HttpRequest, HttpResponse};
use uuid::Uuid;

//Pour s'enregistrer en tant que user

#[post("/login")]
pub async fn sign_in_basic(
    credentials: web::Json<UserAuthentication>,
) -> Result<HttpResponse, ApiError> {
    let credentials = credentials.into_inner();

    //Verifie si le mail existe

    let user = User::find_by_mail(credentials.mail).map_err(|e| match e.status_code {
        404 => ApiError::new(401, "Credentials not valid!".to_string()),
        _ => e,
    })?;

    if user.password.is_none() {
        // l admin utilise keycloack
        return Err(ApiError::new(401, "Credentials not valid!".to_string()));
    }

    //Verifie si le password est ok

    let is_valid = verify_password(
        user.password.as_ref().unwrap(),
        credentials.password.as_bytes(),
    )?;

    if is_valid {
        let user = UserEnvoye::from_user(user); //Convertion vers la bonne structure

        let my_claims = Claims::new_user(&user, 0, false); //Creation du corps du token ici authenf classique

        let token = Claims::create_jwt(&my_claims)?; //Creation du jwt

        let tok = "Bearer ".to_string() + &token;

        Ok(HttpResponse::Ok()
            .insert_header(("Authorization", tok))
            .insert_header(("Access-Control-Expose-Headers", "Authorization")) //rendre le header visible pour le front
            .json(user))
    } else {
        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }
}

#[post("/login/otp")]
async fn double_authentication(
    req: HttpRequest,
    credentials: web::Json<CodeOtp>,
) -> Result<HttpResponse, ApiError> {
    let claims: Claims = Claims::verify_user_session_first(req)?; //verifie legitimite du user et que 2fa activee

    let cred = credentials.into_inner();

    let user = User::verification_2fa(claims.mail, cred.code)?; //verification du code envoye par le user pour le 2FA

    let change = user.change; //recupere le changement de mdp

    let my_claims = Claims::new_user(&user, 0, change.unwrap()); //Creation du corps du token, true car 2FA etablie

    let token = Claims::create_jwt(&my_claims)?; //Creation du jwt

    let tok = "Bearer ".to_string() + &token;

    Ok(HttpResponse::Ok()
        .insert_header(("Authorization", tok))
        .insert_header(("Access-Control-Expose-Headers", "Authorization"))
        .json(user))
}

#[post("/login/extern")]
async fn authentication_ext(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let mail = Keycloak::get_token(req)?;

    let user = User::find_by_mail(mail)?;

    let user = UserEnvoye::from_user(user); //Convertion vers la bonne structure

    let my_claims = Claims::new_user(&user, 1, true); //Creation du corps du token, true car 2FA etablie

    let token = Claims::create_jwt(&my_claims)?; //Creation du jwt

    let tok = "Bearer ".to_string() + &token;

    Ok(HttpResponse::Ok()
        .insert_header(("Authorization", tok))
        .insert_header(("Access-Control-Expose-Headers", "Authorization"))
        .json(user))
}

#[post("/users")]
async fn create_user(user: web::Json<UserRecu>) -> Result<HttpResponse, ApiError> {
    //Enregistre un user

    let user = user.into_inner();

    let _claims: Claims = Claims::verify_admin_session_complete(&user.claim)?;

    let user = User::create(user)?;
    Ok(HttpResponse::Ok().json(user))
}

#[patch("/users/{id}")]
async fn patch_user(
    id: web::Path<Uuid>,
    cred: web::Json<UserChangeCred>,
) -> Result<HttpResponse, ApiError> {
    //Un user peut modifier ses creds

    let cred = cred.into_inner();

    let id = id.into_inner();

    let claims: Claims = Claims::verify_user_session_ext(&cred.claim)?; //verifie legitimite du user

    if claims.id == id && claims.method == 0 {
        //c'est bien le user lui meme qui veut changer ses creds et que 2FA est active

        User::update_password(id, cred)?;
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ApiError::new(403, "Interdit".to_string()))
    }
}

#[post("/users/{id}/otp")]
async fn create_otp_user(
    id: web::Path<Uuid>,
    cred: web::Json<UserChangeCred>,
) -> Result<HttpResponse, ApiError> {
    //Un user peut ajouter la 2FA a son compte

    let cred = cred.into_inner();

    let id = id.into_inner();

    let claims: Claims = Claims::verify_user_session_ext(&cred.claim)?; //verifie legitimite user

    if claims.id == id && claims.method == 0 {
        //c'est bien l'admin lui meme qui veut activer la mfa

        User::create_otp(id, cred.password)?;

        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ApiError::new(403, "Interdit".to_string()))
    }
}

#[delete("/users/{id}")]
async fn delete_user(
    id: web::Path<Uuid>,
    cred: web::Json<UserSupprimer>,
) -> Result<HttpResponse, ApiError> {
    //Un admin peut modifier ses creds

    let cred = cred.into_inner();

    let id = id.into_inner();

    let _claims = Claims::verify_admin_session_complete(&cred.claim)?; //verifie legitimite admin

    User::delete(id)?;

    Ok(HttpResponse::Ok().finish())
}

pub fn routes_user(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_in_basic);
    cfg.service(double_authentication);
    cfg.service(create_user);
    cfg.service(patch_user);
    cfg.service(create_otp_user);
    cfg.service(delete_user);
}
