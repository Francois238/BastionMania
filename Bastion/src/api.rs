use actix_web::{post, Responder, web};
use crate::{persistance, wgconfigure, WGPeerConfig, WGPeerPublicKey};

static WG_INT: &str = "wg-client";

#[post("/adduser")]
async fn add_user(user_config: web::Json<WGPeerConfig>) -> impl Responder {
    let user_config = user_config.into_inner();
    //TODO Validate input
    let res = wgconfigure::add_peer(WG_INT, &user_config);
    if let Err(e) = res {
        return e.to_string();
    }
    let res = persistance::add_peer(&user_config);
    if let Err(e) = res {
        return e.to_string();
    }

    "success".to_string()
}

#[post("/deluser")]
async fn del_user(user_config: web::Json<WGPeerPublicKey>) -> impl Responder {
    let public_key = user_config.into_inner().public_key;
    //TODO Validate input
    let res = wgconfigure::remove_peer(WG_INT, &public_key);
    if let Err(e) = res {
        return e.to_string();
    }
    let res = persistance::remove_peer(public_key);
    if let Err(e) = res {
        return e.to_string();
    }

    "success".to_string()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(add_user)
        .service(del_user);
}