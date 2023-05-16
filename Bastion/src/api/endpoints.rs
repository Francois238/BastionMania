use actix_web::{delete, post, web, HttpResponse, Responder};

use crate::database::BastionDatabase;
use crate::ssh::ressource::SSHRessource;
use crate::ssh::user::SSHUser;
use crate::wireguard::{persistance, wgconfigure};
use crate::{WGPeerConfig, WGPeerPublicKey};

use log::error;

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

    let res = ressource
        .realise()
        .map_err(|e| HttpResponse::InternalServerError().body(e));
    if let Err(e) = res {
        return e;
    }
    let res = ressource
        .save()
        .map_err(|e| HttpResponse::InternalServerError().body(e));
    if let Err(e) = res {
        return e;
    }

    HttpResponse::Ok().body("success")
}

#[post("/ssh/ressources/{ressource_id}/users")]
async fn add_ssh_user(ressource_id: web::Path<String>, user: web::Json<SSHUser>) -> impl Responder {
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

#[delete("/ssh/ressources/{ressource_id}/users/{user_id}")]
async fn remove_ssh_user(path: web::Path<(String, String)>) -> impl Responder {
    let (ressource_id, user_id) = path.into_inner();

    println!("Removing user from ressource: {}", ressource_id);
    println!("User id: {}", user_id);
    //TODO Validate input

    let database = BastionDatabase::get();
    let mut database = match database {
        Ok(d) => d,
        Err(_) => {
            error!("Error loading database");
            return HttpResponse::InternalServerError().body("Error loading database");
        }
    };

    let ressource = database.get_ssh_mut_by_name(&ressource_id);
    let ressource = match ressource {
        Some(r) => r,
        None => {
            error!("Ressource not found : {}", ressource_id);
            return HttpResponse::NotFound().body("Ressource not found");
        }
    };

    let res = ressource.remove_user(&user_id);
    if let Err(e) = res {
        error!("Error removing user: {}", e);
        return HttpResponse::InternalServerError().body("Error removing user");
    }

    let res = database.save();
    if let Err(e) = res {
        error!("Error saving database: {}", e);
        return HttpResponse::InternalServerError().body("Error saving database");
    }

    HttpResponse::Ok().body("success")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_user)
        .service(del_user)
        .service(add_ssh_ressource)
        .service(add_ssh_user)
        .service(remove_ssh_user);
}
