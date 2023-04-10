use actix_web::{HttpResponse, post, Responder, web};

use crate::{persistance, ssh, wgconfigure, WGPeerConfig, WGPeerPublicKey};
use crate::ssh::ressource::SSHRessource;
use crate::ssh::user::SSHUser;

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

#[post("/ssh/ressources")]
async fn add_ssh_ressource(ressource: web::Json<SSHRessource>) -> impl Responder {
    let ressource = ressource.into_inner();
    //TODO Validate input
    let res = ressource.save();
    if let Err(e) = res {
        return e.to_string();
    }

    "success".to_string()
}

#[post("/ssh/ressources/{ressource_name}/users")]
async fn add_ssh_user(ressource_name: web::Path<String>, user: web::Json<SSHUser>) -> impl Responder {
    let ressource_name = ressource_name.into_inner();
    let user = user.into_inner();

    println!("Adding user to ressource: {}", ressource_name);
    println!("{:?}", user);
    //TODO Validate input

    let ressource = SSHRessource::from_name(&ressource_name);
    let ressource = match ressource {
        Ok(r) => r,
        Err(e) => return HttpResponse::NotFound().body("Ressource not found"),
    };

    if ressource.add_user(&user).is_err() {
        return HttpResponse::InternalServerError().body("Error adding user");
    }

    HttpResponse::Ok().body("success")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(add_user)
        .service(del_user)
        .service(add_ssh_ressource)
        .service(add_ssh_user);
}