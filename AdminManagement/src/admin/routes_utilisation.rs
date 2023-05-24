use crate::admin::*;
use crate::tools::api_error::ApiError;
use crate::tools::claims::Claims;

use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse};
use google_authenticator::{ErrorCorrectionLevel, GoogleAuthenticator};
use uuid::Uuid;

//Pour s'enregistrer en tant qu'admin

#[get("/admins")]
async fn find_all_admins(req: HttpRequest, mail: web::Query<MailAdmin>) -> Result<HttpResponse, ApiError> {
    //Recupere la liste des admins

    let _claims = Claims::verify_admin_session_complete(req)?;

    if let Some(mail) = &mail.mail {
        let admin = Admin::find_by_mail_pattern(mail.to_string())?;
        return Ok(HttpResponse::Ok().json(admin));
    }

    let admins = Admin::find_all()?;

    Ok(HttpResponse::Ok().json(admins)) //Retourne la liste
}

#[get("/admins/{id}")]
async fn find_admin(req: HttpRequest, id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    //Recupere un admin

    let _claims = Claims::verify_admin_session_complete(req)?;

    let admin = Admin::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(admin)) //Retourne l'admin
}

#[post("/admins")]
async fn create_admin(
    req: HttpRequest,
    admin: web::Json<AdminReceived>,
) -> Result<HttpResponse, ApiError> {
    //Enregistre un admin

    let claims = Claims::verify_admin_session_complete(req)?;

    let admin = admin.into_inner();

    let admin = AdminSent::from_admin_received(admin); //Creation de la structure envoyee au micro service authentification

    send_admin_to_authentication(&admin, claims).await?; //Envoie l'admin au micro service authentification

    let admin = Admin::create(admin)?; //Insertion de l'admin en bdd

    Ok(HttpResponse::Ok().json(admin))
}

#[post("/admins/{id}/otp")]
async fn ajout_2fa(req: HttpRequest, id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    //Fct pour ajouter la 2fa

    let claims = Claims::verify_admin_session_simple(req)?;

    let mail = claims.mail.clone();

    let id = id.into_inner();

    //Si c'est bien l'admin qui est connecte

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

#[patch("/admins/{id}")]
async fn update(
    req: HttpRequest,
    id: web::Path<Uuid>,
    cred: web::Json<AdminChangeCred>,
) -> Result<HttpResponse, ApiError> {
    //Fct pour mettre a jour donnée de l'admin

    let claims = Claims::verify_admin_session_simple(req)?;

    let id = id.into_inner();

    let cred = cred.into_inner();

    if claims.id == id &&  claims.otp == Some(true) {
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

#[delete("/admins/{id}")]
async fn delete_admin(req: HttpRequest, id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    //Supprime un admin

    let claims = Claims::verify_admin_session_complete(req)?;

    let id = id.into_inner();

    delete_admin_to_authentication(id, claims).await?; //Envoie de l'id de l'admin au micro service authentification

    let _d = Admin::delete(id)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/premiere_utilisation")]
async fn premiere_utilisation(admin: web::Json<AdminReceived>) -> Result<HttpResponse, ApiError> {
    //Enregistre un admin

    let admin = admin.into_inner();

    let admin = AdminSent::from_admin_received(admin); //Creation de la structure envoyee au micro service authentification

    first_use_to_authentication(&admin).await?; //Envoie l'admin au micro service authentification

    let admin = premiere_utilisation_bastion(admin)?; //Insertion de l'admin en bdd

    Ok(HttpResponse::Ok().json(admin))
}

pub fn routes_admin_utilisation(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all_admins);
    cfg.service(find_admin);
    cfg.service(create_admin);
    cfg.service(ajout_2fa);
    cfg.service(update);
    cfg.service(delete_admin);
    cfg.service(premiere_utilisation);
}
