use actix_web::{post, web, HttpResponse, Responder};


use crate::ssh::ressource::SSHRessource;
use crate::ssh::user::SSHUser;
use crate::wireguard::{persistance, wgconfigure};
use crate::{WGPeerConfig, WGPeerPublicKey};

static WG_INT: &str = "wg-client";

#[post("/adduser")]
async fn add_user(user_config: web::Json<WGPeerConfig>) -> impl Responder {
    let user_config = user_config.into_inner();
    //TODO Validate input
    let res = wgconfigure::add_peer(WG_INT, &user_config);
    if let Err(e) = res {
        return e;
    }
    let res = persistance::add_peer(&user_config);
    if let Err(e) = res {
        return e;
    }

    "success".to_string()
}

#[post("/deluser")]
async fn del_user(user_config: web::Json<WGPeerPublicKey>) -> impl Responder {
    let public_key = user_config.into_inner().public_key;
    //TODO Validate input
    let res = wgconfigure::remove_peer(WG_INT, &public_key);
    if let Err(e) = res {
        return e;
    }
    let res = persistance::remove_peer(public_key);
    if let Err(e) = res {
        return e;
    }

    "success".to_string()
}

#[post("/ssh/ressources")]
async fn add_ssh_ressource(ressource: web::Json<SSHRessource>) -> HttpResponse {
    let ressource = ressource.into_inner();
    //TODO Validate input

    let res = ressource.realise().map_err(|e| HttpResponse::InternalServerError().body(e));
    if let Err(e) = res {
        return e;
    }
    let res = ressource.save().map_err(|e| HttpResponse::InternalServerError().body(e));
    if let Err(e) = res {
        return e;
    }

    HttpResponse::Ok().body("success")
}

#[post("/ssh/ressources/{ressource_id}/users")]
async fn add_ssh_user(
    ressource_id: web::Path<String>,
    user: web::Json<SSHUser>,
) -> impl Responder {
    let ressource_id = ressource_id.into_inner();
    let user = user.into_inner();

    println!("Adding user to ressource: {}", ressource_id);
    println!("{:?}", user);
    //TODO Validate input

    let ressource = SSHRessource::from_name(&ressource_id);
    let ressource = match ressource {
        Ok(r) => r,
        Err(_) => return HttpResponse::NotFound().body("Ressource not found"),
    };

    if ressource.add_user(&user).is_err() {
        return HttpResponse::InternalServerError().body("Error adding user");
    }

    HttpResponse::Ok().body("success")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_user)
        .service(del_user)
        .service(add_ssh_ressource)
        .service(add_ssh_user);
}
