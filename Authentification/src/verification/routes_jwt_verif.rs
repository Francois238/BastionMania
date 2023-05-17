use actix_web::{get, HttpResponse, web};

use crate::{tools::{ApiError, Claims}, verification::Token};




#[get("/verification/user")]
async fn verif_user(token: web::Json<Token>) -> Result<HttpResponse, ApiError> {
    //Enregistre un user

    let token = token.into_inner();

    let jwt = token.jwt;

    let _claims: Claims = Claims::verify_user_session_complete(&jwt)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/verification/admin")]
async fn verif_admin(token: web::Json<Token>) -> Result<HttpResponse, ApiError> {
    //Enregistre un user

    let token = token.into_inner();

    let jwt = token.jwt;

    let _claims: Claims = Claims::verify_admin_session_complete(&jwt)?;

    Ok(HttpResponse::Ok().finish())
}

pub fn routes_verification(cfg: &mut web::ServiceConfig) {
    cfg.service(verif_user);
    cfg.service(verif_admin);
}