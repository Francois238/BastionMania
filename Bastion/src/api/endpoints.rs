use std::fs;
use actix_web::{delete, post, get, web, HttpResponse, Responder};

use crate::database::BastionDatabase;
use crate::ssh::ressource::SSHRessource;
use crate::ssh::user::SSHUser;
use crate::{WireguardAgent, WireguardRessource};

use log::{error, info};
use wireguard_keys::Privkey;
use crate::consts::WG_PRIVATE_KEY_PATH;
use crate::wireguard::wgconfigure;


/// Return wireguard public_key
#[get("/wireguard/public_key")]
async fn get_wireguard_public_key() -> impl Responder {
    let res = wgconfigure::get_public_key();
    match res {
        Ok(key) => HttpResponse::Ok().body(key),
        Err(e) => {
            error!("Error getting public key: {}", e);
            HttpResponse::InternalServerError().body("Error getting public key")
        }
    }
}

#[post("/wireguard/configs")]
async fn add_wireguard_config(user_config: web::Json<WireguardRessource>) -> impl Responder {
    let user_config = user_config.into_inner();
    //TODO Validate input
    let database = BastionDatabase::get();
    let mut database = match database {
        Ok(d) => d,
        Err(_) => {
            error!("Error loading database");
            return HttpResponse::InternalServerError().body("Error loading database");
        }
    };

    if database.wireguard_exists(user_config.clone()) {
        error!("User already exists : {:?}", user_config);
        return HttpResponse::BadRequest().body("User already exists");
    }

    let res = user_config.create();
    if let Err(e) = res {
        error!("Error creating config: {}", e);
        return HttpResponse::InternalServerError().body("Error creating config");
    }

    let res = database.add_wireguard(user_config);
    if let Err(e) = res {
        error!("Error adding config to database: {}", e);
        return HttpResponse::InternalServerError().body("Error adding config to database");
    }

    HttpResponse::Ok().body("success")
}

#[delete("/wireguard/configs/{res_id}/{client_id}")]
async fn remove_wireguard_config(path: web::Path<(String, String)>) -> impl Responder {
    let (res_id, client_id) = path.into_inner();
    //TODO Validate input

    let database = BastionDatabase::get();
    let mut database = match database {
        Ok(d) => d,
        Err(_) => {
            error!("Error loading database");
            return HttpResponse::InternalServerError().body("Error loading database");
        }
    };

    let user_config = match database.get_wireguard_ressource(&res_id, &client_id) {
        Some(r) => r,
        None => {
            error!("Config not found : {} {}", res_id, client_id);
            return HttpResponse::NotFound().body("Config not found");
        }
    };
        
    let res = user_config.delete();
    if let Err(e) = res {
        error!("Error deleting config: {}", e);
        return HttpResponse::InternalServerError().body("Error deleting config");
    }

    let res = database.remove_wireguard(&res_id, &client_id);
    if let Err(e) = res {
        error!("Error deleting config from database: {}", e);
        return HttpResponse::InternalServerError().body("Error deleting config from database");
    }

    HttpResponse::Ok().body("success")
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

    info!("Adding user to ressource: {}", ressource_id);
    info!("{:?}", user);
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

    info!(
        "Removing user : {} from ressource: {}",
        user_id, ressource_id
    );
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

/// Remove a ressource
///
/// This will remove the ressource from the database and delete the user from the system
#[delete("/ssh/ressources/{ressource_id}")]
async fn remove_ssh_ressource(ressource_id: web::Path<String>) -> impl Responder {
    let ressource_id = ressource_id.into_inner();

    info!("Removing ressource: {}", ressource_id);
    //TODO Validate input

    let database = BastionDatabase::get();
    let mut database = match database {
        Ok(d) => d,
        Err(_) => {
            error!("Error loading database");
            return HttpResponse::InternalServerError().body("Error loading database");
        }
    };

    let ressource = database.get_ssh_by_name(&ressource_id);
    let ressource = match ressource {
        Some(r) => r,
        None => {
            error!("Ressource not found : {}", ressource_id);
            return HttpResponse::NotFound().body("Ressource not found");
        }
    };

    let res = ressource.delete();
    if let Err(e) = res {
        error!("Error deleting ressource: {}", e);
        return HttpResponse::InternalServerError().body("Error deleting ressource");
    }

    let res = database.remove_ssh_by_name(&ressource_id);
    if let Err(e) = res {
        error!("Error removing ressource: {}", e);
        return HttpResponse::InternalServerError().body("Error removing ressource");
    }

    let res = database.save();
    if let Err(e) = res {
        error!("Error saving database: {}", e);
        return HttpResponse::InternalServerError().body("Error saving database");
    }

    HttpResponse::Ok().body("success")
}

#[post("/agent")]
async fn set_agent(agent: web::Json<WireguardAgent>) -> HttpResponse{
    let agent = agent.into_inner();
    let database = BastionDatabase::get();
    let mut database = match database {
        Ok(d) => d,
        Err(_) => {
            error!("Error loading database");
            return HttpResponse::InternalServerError().body("Error loading database");
        }
    };

    let res = wgconfigure::configure_to_agent(&agent);
    if let Err(e) = res {
        error!("Error configuring agent: {}", e);
        return HttpResponse::InternalServerError().body("Error configuring agent");
    }

    let res = database.set_agent(agent);
    if let Err(e) = res {
        error!("Error saving agent: {}", e);
        return HttpResponse::InternalServerError().body("Error saving agent");
    }

    let bastion_private_key = match fs::read_to_string(WG_PRIVATE_KEY_PATH){
        Ok(k) => k,
        Err(e) => {
            error!("Error reading private key: {}", e);
            return HttpResponse::InternalServerError().body("Error reading private key");
        }
    };
    let bastion_private_key = match Privkey::from_base64(&bastion_private_key){
        Ok(k) => k,
        Err(e) => {
            error!("Error parsing private key: {}", e);
            return HttpResponse::InternalServerError().body("Error parsing private key");
        }
    };

    HttpResponse::Ok().body(bastion_private_key.pubkey().to_base64())
}



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_wireguard_public_key)
        .service(add_wireguard_config)
        .service(remove_wireguard_config)
        .service(add_ssh_ressource)
        .service(add_ssh_user)
        .service(remove_ssh_user)
        .service(remove_ssh_ressource)
        .service(set_agent);
}
