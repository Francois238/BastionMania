use std::borrow::BorrowMut;
use std::env;
use actix_web::{get,
                post,
                patch,
                delete,
                error::ResponseError,

                Responder,
                HttpResponse,
                http::{header::ContentType, StatusCode}, web
};

use crate::api::*;
use crate::api_error::ApiError;
use crate::db;
use crate::services::{generate_bastion_freenetid, generate_bastion_freeport, generate_user_freenetid};
//use derive_more::{Display};
use serde::{Serialize, Deserialize};
use serde_json::json;
use actix_session::Session;
use repository::*;
use crate::entities::{Bastion, BastionInsertable, Users, UsersModification};
use crate::model::{BastionInstanceCreate, BastionModification, ConfigAgent, RetourAPI, ConfigUser, UsersCreation, Claims, ConfigClient, InstanceClient};


// /bastion =======================================================================================

#[get("/bastion")]
pub async fn get_bastion() -> Result<HttpResponse, ApiError>{
    let bastion_insere = Bastion::find_all()?;
    Ok(HttpResponse::Ok().json(bastion_insere))
}

#[post("/bastion")]
pub async fn create_bastion( bastion: web::Json<BastionModification>, session: Session) -> Result<HttpResponse, ApiError>{

    let _claims = Claims::verifier_session_admin(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    //generation pair de clé pour agent et bastion
    let agent_priv = wireguard_keys::Privkey::generate();
    let agent_pub = agent_priv.pubkey();
    let bastion_priv = wireguard_keys::Privkey::generate();
    let bastion_pub = bastion_priv.pubkey();

    let liste_bastions=Bastion::find_all()?;
    //generation d'un port libre pour le bastion
    let ports: Vec<i32> = (&liste_bastions).into_iter().map(|b| b.port ).collect();
    let port = generate_bastion_freeport(&ports);
    //generation d'un net_id libre pour le bastion
    let net_ids: Vec<i32> = liste_bastions.into_iter().map(|b| b.net_id ).collect();
    let net_id = generate_bastion_freenetid(&net_ids);

    //creation du bastion
    let bastion_insertion = BastionInsertable{
        name: bastion.name.clone(),
        subnet_cidr: bastion.subnet_cidr.clone(),
        agent_endpoint: bastion.agent_endpoint.clone(),
        pubkey: bastion_pub.to_base64(),
        port,
        net_id,
    };

    let bastion_insere = Bastion::create(bastion_insertion)?;

    let bastion_instance_create = BastionInstanceCreate {
        privkey: bastion_priv.to_base64(),
        subnet_cidr: bastion.subnet_cidr.clone(),
        agent_pubkey: agent_pub.to_base64(),
        agent_endpoint: bastion.agent_endpoint.clone(),
        net_id,
    };

    //TODO: envoyer la requete de creation de bastion a l'intancieur

    let retour_api= RetourAPI{
        success: true,
        message:  "Bastion créé".to_string(),
        data: ConfigAgent{
            privkey: agent_priv.to_base64(),
            pubkey: bastion_pub.to_base64(),
        }
    };

    Ok(HttpResponse::Ok().json(retour_api))
    
}

// /bastion/{bastion_id} ==========================================================================

#[get("/bastion/{bastion_id}")]
pub async fn find_a_bastion(bastion_id:  web::Path<i32>, session: Session) -> Result<HttpResponse, ApiError>{
    let claims = Claims::verifier_session_user(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;
    let bastion_id = bastion_id.into_inner();
    let authorisation = Bastion::verification_appartenance( claims.id, bastion_id).map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
    
    let bastion_affiche = Bastion::find_un_bastion(bastion_id)?;
    Ok(HttpResponse::Ok().json(bastion_affiche))

}

#[patch("/bastion/{bastion_id}")]
pub async fn update_a_bastion(bastion_id:web::Path<i32>, modifs:web::Json<BastionModification>, session: Session) -> Result<HttpResponse, ApiError>{
    let _claims = Claims::verifier_session_admin(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;
    let bastion_modif = Bastion::update_un_bastion(bastion_id.into_inner(), modifs.into_inner())?;
    Ok(HttpResponse::Ok().json(bastion_modif))
}

#[delete("/bastion/{bastion_id}")]
pub async fn delete_a_bastion(bastion_id:web::Path<i32>, session: Session) -> Result<HttpResponse, ApiError>{
    let _claims = Claims::verifier_session_admin(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;
    let bastion_suppr = Bastion::delete_un_bastion(bastion_id.into_inner())?;
    Ok(HttpResponse::Ok().json("supprimé"))
}

//  /bastion/{bastion_id}/users ===================================================================

#[get("/bastion/{bastion_id}/users")]
pub async fn get_users(bastion_id: web::Path<i32>, session: Session) -> Result<HttpResponse, ApiError>{
    let _claims = Claims::verifier_session_admin(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;
    let users = Users::find_users_bastion(bastion_id.into_inner())?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("/bastion/{bastion_id}/users")]
pub async fn create_users(users: web::Json<UsersCreation>, bastion_id:web::Path<i32>, session: Session ) -> Result<HttpResponse, ApiError>{

    let _claims = Claims::verifier_session_admin(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    let bastion_id = bastion_id.into_inner();

    let liste_users=Users::find_users_bastion(bastion_id)?;

    let net_ids: Vec<i32> = liste_users.into_iter().map(|b| b.net_id ).collect();
    let net_id = generate_user_freenetid(&net_ids);

    let users_insertion = Users {
        id: users.id.clone(),
        bastion_id,
        wireguard: false,
        net_id,
    };

    let users = Users::create_users( users_insertion)?;

    let retour_api= RetourAPI{
        success: true,
        message:  "Bastion créé".to_string(),
        data: ConfigUser{
            id: users.id.clone(),
            net_id,
        }
    };


    Ok(HttpResponse::Ok().json(retour_api))
}

// /bastion/{bastion_id}/users/{user_id} =========================================================

#[get("/bastion/{bastion_id}/users/{user_id}")]
pub async fn get_a_user(user_id: web::Path<i32>, session: Session) -> Result<HttpResponse, ApiError>{
    let _claims = Claims::verifier_session_admin(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;
    let users = Users::find_un_user(user_id.into_inner())?;
    Ok(HttpResponse::Ok().json(users))
}

#[delete("/bastion/{bastion_id}/users/{user_id}")]
pub async fn delete_a_user(user_id: web::Path<i32>, session: Session) -> Result<HttpResponse, ApiError>{
    let _claims = Claims::verifier_session_admin(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;
    let user_suppr = Users::delete_un_user(user_id.into_inner())?;
    Ok(HttpResponse::Ok().json("supprimé"))
}

// /bastion/{bastion_id}/users/{user_id}/generate_wireguard" ===============================================

#[get("/bastion/{bastion_id}/users/{user_id}/generate_wireguard")]
pub async fn get_user_wireguard_status(session: Session, bastion_id: web::Path<i32>, user_id: web::Path<i32>) -> Result<HttpResponse, ApiError>{
    let _claims = Claims::verifier_session_user(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    let bastion_id = bastion_id.into_inner();
    let user_id = user_id.into_inner();

    let client_priv = wireguard_keys::Privkey::generate();
    let client_pub = client_priv.pubkey();

    let bastion_ip = bastion_ip: env::var("BASTION_IP").expect("BASTION_IP must be set");

    let client_address= build_client_address(bastion_id, user_id)?;
    let bastion_endpoint = build_endpoint_user(bastion_ip, bastion_id)?;

    let client_public_key = client_pub.to_string();
    let client_private_key = client_priv.to_string();

    let instance_client = InstanceClient{
        client_public_key,
        client_address,
    };

    //TODO instancier le client dans l'instancieur

    update_un_user(user_id)?;

    let bastion_public_key = get_bastion_public_key(bastion_id)?;
    let subnet_cidr = get_bastion_subnet_cidr(bastion_id)?;

    RetourAPI{
        success: true,
        message:  "accés client créé".to_string(),
        data: ConfigClient{
            client_private_key,
            client_address, //TODO mettre en mutable
            bastion_public_key,
            bastion_endpoint,
            subnet_cidr

        }
    };

    Ok(HttpResponse::Ok().json(retour_api))
}

pub fn routes_bastion(cfg: &mut web::ServiceConfig) {
    cfg.service(create_bastion);
    cfg.service(get_bastion);

    cfg.service(update_a_bastion);
    cfg.service(find_a_bastion);
    cfg.service(delete_a_bastion);

    cfg.service(get_users);
    cfg.service(create_users);

    cfg.service(get_a_user);
    cfg.service(delete_a_user);
/*
    cfg.service(find_server_config); 
    cfg.service(update_server_config); */
}
    
   

