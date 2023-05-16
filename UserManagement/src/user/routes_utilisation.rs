use crate::tools::api_error::ApiError;
use crate::tools::claims::Claims;
use crate::user::*;

use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse};
use google_authenticator::{ErrorCorrectionLevel, GoogleAuthenticator};
use uuid::Uuid;

#[get("/users")]
async fn find_all_users(
    req: HttpRequest,
    mail: web::Query<MailUser>,
) -> Result<HttpResponse, ApiError> {
    //Recupere la liste des users

    let _claims = Claims::verify_admin_session_complete(req)?;

    if let Some(mail) = &mail.mail {
        let users = User::find_by_mail_pattern(mail.to_string())?;
        return Ok(HttpResponse::Ok().json(users)); //Retourne la liste
    }

    let users = User::find_all()?;

    Ok(HttpResponse::Ok().json(users)) //Retourne la liste
}

#[get("/users/{id}")]
async fn find_user(req: HttpRequest, id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    //Recupere un user

    let _claims = Claims::verify_admin_session_complete(req)?;

    let user = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user)) //Retourne l'user
}

#[post("/users")]
async fn create_user(
    req: HttpRequest,
    user: web::Json<UserReceived>,
) -> Result<HttpResponse, ApiError> {
    //Enregistre un user

    let claims = Claims::verify_admin_session_complete(req)?;

    let user = user.into_inner();

    let user = UserSent::from_user_received(user); //Creation de la structure envoyee au micro service authentification

    send_user_to_authentication(&user, claims).await?; //Envoie le user au micro service authentification

    let user = User::create(user)?; //Insertion de l'user en bdd

    Ok(HttpResponse::Ok().json(user))
}

#[patch("/users/{id}")]
async fn update(
    req: HttpRequest,
    id: web::Path<Uuid>,
    cred: web::Json<UserChangeCred>,
) -> Result<HttpResponse, ApiError> {
    //Fct pour mettre a jour donnée de l'admin

    let claims = Claims::verify_user_session_simple(req)?;

    let id = id.into_inner();

    let cred = cred.into_inner();

    if claims.id == id && claims.otp == Some(true) {
        send_password_to_authentication(cred.password, &claims).await?; //Envoie du nouveau mdp au micro service authentification

        let mut claims = claims;

        claims.complete_authentication = true;

        let jwt = Claims::create_jwt(&claims)?;

        let tok = "Bearer ".to_string() + &jwt;

        Ok(HttpResponse::Ok()
            .insert_header(("Authorization", tok))
            .insert_header(("Access-Control-Expose-Headers", "Authorization"))
            .finish())
    } else {
        Err(ApiError::new(403, "Interdit".to_string()))
    }
}

#[post("/users/{id}/otp")]
async fn ajout_2fa(req: HttpRequest, id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    //Fct pour activer la 2fa

    let claims = Claims::verify_user_session_simple(req)?;

    let mail = claims.mail.clone();

    //le user a jamais active double authenf

    let id = id.into_inner();

    if claims.id == id && !claims.complete_authentication && claims.otp == Some(false) {
        let auth = GoogleAuthenticator::new();

        let secret = auth.create_secret(32);

        send_otp_to_authentication(secret.clone(), claims).await?; //Envoie de la secret key au micro service authentification

        let url = auth.qr_code_url(
            &secret,
            &mail,
            "bastion_mania",
            200,
            200,
            ErrorCorrectionLevel::High,
        );

        let code = CodeOtp { url };

        Ok(HttpResponse::Ok().json(code))
    } else {
        Err(ApiError::new(403, "Interdit".to_string()))
    }
}

#[delete("/users/{id}")]
async fn delete_user(req: HttpRequest, id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    //Supprime un user

    let claims = Claims::verify_admin_session_complete(req)?;

    let id = id.into_inner();

    delete_user_to_authentication(id, claims).await?; //Envoie de l'id de l'admin au micro service authentification

    let _d = User::delete(id)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("extern/users")]
async fn add_user_extern(
    user: web::Json<UserSentAuthentication>,
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();

    let _ok = Claims::verify_session_add_from_authentication(user.claims.clone())?;

    let user = User::add_user_extern(user)?;

    Ok(HttpResponse::Ok().json(user))
}

pub fn routes_user_utilisation(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all_users);
    cfg.service(find_user);
    cfg.service(create_user);
    cfg.service(ajout_2fa);
    cfg.service(update);
    cfg.service(delete_user);
    cfg.service(add_user_extern);
}
