use actix_web::{
    delete,
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    patch, post, web, HttpResponse, Responder,
};
use std::borrow::BorrowMut;
use std::env;

use crate::api::*;
use crate::api_error::ApiError;
use crate::db;
use crate::services::{generate_bastion_freenetid, generate_bastion_freeport, generate_ressource_freenetid, generate_ressource_k8s_freenetid, generate_ressource_ssh_freenetid, generate_ressource_wireguard_freenetid, generate_user_freenetid};
//use derive_more::{Display};
use crate::entities::{Bastion, BastionInsertable, Ressource, RessourceInsertable, Users, UsersModification};
use crate::model::{
    BastionInstanceCreate, BastionModification, BastionSuppression, Claims, ConfigAgent,
    ConfigClient, ConfigUser, InstanceClient, RetourAPI, UsersCreation, UsersInstanceCreate,
};
use actix_session::Session;
use diesel::dsl::Nullable;
use log::error;
use repository::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::Option;
use serde_json::json;
use tracing::info;
use crate::model::ressourcemodification::RessourceCreation;

// /bastion =======================================================================================

#[get("/bastions")]
pub async fn get_bastion(session: Session) -> Result<HttpResponse, ApiError> {
    let claims_admin = Claims::verifier_session_admin(&session);

    match claims_admin {
        Some(claims) => {
            let bastion_insere = Bastion::find_all()?;
            Ok(HttpResponse::Ok().json(bastion_insere))
        }
        None => {
            let claims = Claims::verifier_session_user(&session)
                .ok_or(ApiError::new(404, "Not Found".to_string()))
                .map_err(|e| e)?;
            let users = Bastion::bastion_user(claims.id)?;
            let mut bastions: Vec<Bastion> = Vec::new();

            for user in users {
                let bastion = Bastion::find_un_bastion(user.bastion_id)?;
                bastions.push(bastion);
            }

            Ok(HttpResponse::Ok().json(bastions))
        }
    }
}

#[post("/bastions")]
pub async fn create_bastion(
    bastion: web::Json<BastionModification>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let _claims = Claims::verifier_session_admin(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;

    //generation pair de clé pour agent et bastion
    let agent_priv = wireguard_keys::Privkey::generate();
    let agent_pub = agent_priv.pubkey();
    let bastion_priv = wireguard_keys::Privkey::generate();
    let bastion_pub = bastion_priv.pubkey();

    let liste_bastions = Bastion::find_all()?;
    //generation d'un port libre pour le bastion
    let ports: Vec<i32> = (&liste_bastions).into_iter().map(|b| b.port).collect();
    let port = generate_bastion_freeport(&ports);
    //generation d'un net_id libre pour le bastion
    let net_ids: Vec<i32> = liste_bastions.into_iter().map(|b| b.net_id).collect();
    let net_id = generate_bastion_freenetid(&net_ids);

    //creation du bastion
    let bastion_insertion = BastionInsertable {
        name: bastion.name.clone(),
        subnet_cidr: bastion.subnet_cidr.clone(),
        agent_endpoint: bastion.agent_endpoint.clone(),
        pubkey: bastion_pub.to_base64(),
        port,
        net_id,
    };

    let bastion_insere = Bastion::create(bastion_insertion)?;

    let bastion_instance_create = BastionInstanceCreate {
        private_key: bastion_priv.to_base64(),
        cidr_protege: bastion.subnet_cidr.clone(),
        agent_public_key: agent_pub.to_base64(),
        agent_endpoint: bastion.agent_endpoint.clone(),
        bastion_port: port,
        net_id,
    };

    // envoyer la requete de creation de bastion a l'intancieur
    let _client = reqwest::Client::new();
    let endpoint = env::var("INSTANCIEUR_ENDPOINT");
    let endpoint = if let Ok(endpoint) = endpoint {
        endpoint
    } else {
        error!("Endpoint: {}", "Endpoint non défini");
        return Err(ApiError::new(500, "Endpoint non défini".to_string()));
    };

    let url = format!("http://{}/create/{}", endpoint, bastion_insere.id);
    let _response = _client
        .post(&url)
        .json(&bastion_instance_create)
        .send()
        .await
        .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

    let retour_api = RetourAPI {
        success: true,
        message: "Bastion créé".to_string(),
        data: ConfigAgent {
            privkey: agent_priv.to_base64(),
            pubkey: bastion_pub.to_base64(),
        },
    };

    Ok(HttpResponse::Ok().json(retour_api))
}

// /bastion/{bastion_id} ==========================================================================

#[get("/bastions/{bastion_id}")]
pub async fn find_a_bastion(
    bastion_id: web::Path<i32>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let claims_admin = Claims::verifier_session_admin(&session);
    let bastion_id = bastion_id.into_inner();

    match claims_admin {
        Some(_claims) => {
            let bastion_affiche = Bastion::find_un_bastion(bastion_id)?;
            Ok(HttpResponse::Ok().json(bastion_affiche))
        }
        None => {
            let claims = Claims::verifier_session_user(&session)
                .ok_or(ApiError::new(404, "Not Found".to_string()))
                .map_err(|e| e)?;
            let authorisation = Bastion::verification_appartenance(claims.id, bastion_id)
                .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
            if !authorisation {
                return Err(ApiError::new(404, "Not Found".to_string()));
            }
            let bastion = Bastion::find_un_bastion(bastion_id)?;

            Ok(HttpResponse::Ok().json(bastion))
        }
    }
}

#[patch("/bastions/{bastion_id}")]
pub async fn update_a_bastion(
    bastion_id: web::Path<i32>,
    modifs: web::Json<BastionModification>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let _claims = Claims::verifier_session_admin(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;

    let bastion_modification = BastionModification {
        name: modifs.name.clone(),
        subnet_cidr: modifs.subnet_cidr.clone(),
        agent_endpoint: modifs.agent_endpoint.clone(),
    };

    //TODO: envoyer la requete de modification de bastion a l'intancieur

    let bastion_modif = Bastion::update_un_bastion(bastion_id.into_inner(), modifs.into_inner())?;
    Ok(HttpResponse::Ok().json(bastion_modif))
}

#[delete("/bastions/{bastion_id}")]
pub async fn delete_a_bastion(
    bastion_id: web::Path<i32>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let _claims = Claims::verifier_session_admin(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;
    let bastion_id = bastion_id.into_inner();

    let bastion_suppression = BastionSuppression {
        id: bastion_id.clone(),
    };

    let endpoint = env::var("INSTANCIEUR_ENDPOINT");
    let endpoint = if let Ok(endpoint) = endpoint {
        endpoint
    } else {
        error!("Endpoint: {}", "Endpoint non défini");
        return Err(ApiError::new(500, "Endpoint non défini".to_string()));
    };

    // envoyer la requete de suppression de bastion a l'intancieur qui doit aussi approuver la suppression des users
    let _client = reqwest::Client::new();
    let url = format!("http://{}/delete/{}", endpoint, bastion_id);
    let _response = _client
        .post(&url)
        .send()
        .await
        .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

    let _users = Users::delete_all_users(bastion_id)?;

    let bastion_suppr = Bastion::delete_un_bastion(bastion_id)?;
    Ok(HttpResponse::Ok().json("supprimé"))
}

//  /bastion/{bastion_id}/users ===================================================================

#[get("/bastions/{bastion_id}/users")]
pub async fn get_users(
    bastion_id: web::Path<i32>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let _claims = Claims::verifier_session_admin(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;
    let users = Users::find_users_bastion(bastion_id.into_inner())?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("/bastions/{bastion_id}/users")]
pub async fn create_users(
    users: web::Json<UsersCreation>,
    bastion_id: web::Path<i32>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let _claims = Claims::verifier_session_admin(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;

    let bastion_id = bastion_id.into_inner();

    let liste_users = Users::find_users_bastion(bastion_id)?;

    let net_ids: Vec<i32> = liste_users.into_iter().map(|b| b.net_id).collect();
    let net_id = generate_user_freenetid(&net_ids);

    let users_insertion = UsersModification {
        user_id: users.id.clone(),
        bastion_id,
        wireguard: false,
        net_id,
    };

    let users = Users::create_users(users_insertion)?;

    let retour_api = RetourAPI {
        success: true,
        message: "User créé".to_string(),
        data: ConfigUser {
            id: users.user_id.clone(),
            net_id,
        },
    };

    Ok(HttpResponse::Ok().json(retour_api))
}

// /bastion/{bastion_id}/users/{user_id} =========================================================

#[get("/bastions/{bastion_id}/users/{user_id}")]
pub async fn get_a_user(
    données: web::Path<(i32, i32)>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let _claims = Claims::verifier_session_admin(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;
    let (bastion_id, user_id) = données.into_inner();
    let users = Users::find_un_user(bastion_id, user_id)?;
    Ok(HttpResponse::Ok().json(users))
}

#[delete("/bastions/{bastion_id}/users/{user_id}")]
pub async fn delete_a_user(
    données: web::Path<(i32, i32)>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let _claims = Claims::verifier_session_admin(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;
    let (bastion_id, user_id) = données.into_inner();
    let user_suppr = Users::delete_un_user(bastion_id, user_id)?;
    // TODO: envoyer la requete de suppression de user au bastion
    Ok(HttpResponse::Ok().json("supprimé"))
}

// /bastion/{bastion_id}/users/{user_id}/generate_wireguard" ===============================================

#[post("/bastions/{bastion_id}/users/{user_id}/generate_wireguard")]
pub async fn get_user_wireguard_status(
    session: Session,
    donnees: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    info!("request: generation_wireguard");
    let claims = Claims::verifier_session_user(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;
    let (bastion_id, user_id) = donnees.into_inner();
    let authorisation = Bastion::verification_appartenance(claims.id, bastion_id)
        .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
    if !authorisation {
        return Err(ApiError::new(404, "Not Found".to_string()));
    }
    info!(
        "request: generation_wireguard, user_id: {} autorisé",
        user_id
    );
    let client_priv = wireguard_keys::Privkey::generate();
    let client_pub = client_priv.pubkey();

    let bastion_ip = env::var("BASTION_IP").expect("BASTION_IP must be set");

    let client_address = build_client_address(bastion_id, user_id)?;
    let bastion_endpoint = build_endpoint_user(bastion_ip.to_string(), bastion_id)?;

    let client_public_key = client_pub.to_base64();
    let client_private_key = client_priv.to_base64();

    let instance_client = InstanceClient {
        public_key: client_public_key,
        allowed_ips: client_address.clone(),
    };

    //instancier le client dans Bastion
    info!("ajout du peer dans le bastion : {:?}", instance_client);
    let _client = reqwest::Client::new();
    let url = format!("http://intern-bastion-{}:9000/adduser", bastion_id);
    let _response = _client
        .post(&url)
        .json(&instance_client)
        .send()
        .await
        .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

    info!("Peer ajouté au bastion");

    update_un_user(user_id, true)?;

    let bastion_public_key = get_bastion_public_key(bastion_id)?;
    let subnet_cidr = get_bastion_subnet_cidr(bastion_id)?;

    let retour_api = RetourAPI {
        success: true,
        message: "accés client créé".to_string(),
        data: ConfigClient {
            client_private_key,
            client_address,
            bastion_public_key,
            bastion_endpoint,
            subnet_cidr,
        },
    };

    Ok(HttpResponse::Ok().json(retour_api))
}

// /bastion/{bastion_id}/ressources        ===================================================================

#[get("/bastions/{bastion_id}/ressources")]
pub async fn get_ressources(
    bastion_id: web::Path<i32>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let _claims = Claims::verifier_session_admin(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;

    let ressources = Ressource::find_all_ressources(bastion_id.into_inner())?;
    Ok(HttpResponse::Ok().json(ressources))
}
#[post("/bastions/{bastion_id}/ressources")]
pub async fn create_ressources(
    bastion_id: web::Path<i32>,
    nom: web::Json<RessourceCreation>,
    rtype: web::Json<RessourceCreation>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let _claims = Claims::verifier_session_admin(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;
    let bastion_id = bastion_id.into_inner();
    let liste_ressources = Ressource::find_all_ressources(bastion_id)?;

    let ids: Vec<i32> = (&liste_ressources).into_iter().map(|b| b.id).collect();
    let id = generate_ressource_freenetid(&ids);

    let name = nom.into_inner().name;
    let rtype = rtype.into_inner().rtype;

    if rtype=="wireguard"{
        let wids: Vec<i32> = (&liste_ressources).into_iter().filter(|b| b.rtype=="wireguard").map(|b| b.id_wireguard).collect();
        let wid = generate_ressource_wireguard_freenetid(&wids);
        let sid = None;
        let kid = None;

        let ressource_insertion = RessourceInsertable {
            id,
            name,
            rtype,
            id_bastion: bastion_id,
            id_wireguard: Some(wid),
            id_ssh: sid,
            id_k8s: kid,
        };
        let ressources = Ressource::create_ressources(ressource_insertion)?;
        Ok(HttpResponse::Ok().json(ressources))
    }
    else if rtype=="ssh" {
        let sids: Vec<i32> = (&liste_ressources).into_iter().filter(|b| b.rtype=="ssh").map(|b| b.id_ssh).collect();
        let sid = generate_ressource_ssh_freenetid(&sids);
        let wid = None;
        let kid = None;

        let ressource_insertion = RessourceInsertable {
            id,
            name,
            rtype,
            id_bastion: bastion_id,
            id_wireguard: wid,
            id_ssh: Some(sid),
            id_k8s: kid,
        };
        let ressources = Ressource::create_ressources(ressource_insertion)?;
        Ok(HttpResponse::Ok().json(ressources))
    }
    else{
        let kids: Vec<i32> = (&liste_ressources).into_iter().filter(|b| b.rtype=="kubernetes").map(|b| b.id_k8s).collect();
        let kid = generate_ressource_k8s_freenetid(&kids);
        let wid = None;
        let sid = None;

        let ressource_insertion = RessourceInsertable {
            id,
            name,
            rtype,
            id_bastion: bastion_id,
            id_wireguard: wid,
            id_ssh: sid,
            id_k8s: Some(kid),
        };
        let ressources = Ressource::create_ressources(ressource_insertion)?;
        Ok(HttpResponse::Ok().json(ressources))
    }

}

// /bastion/{bastion_id}/ressources/{ressource_id}        ===================================================================

#[get("/bastions/{bastion_id}/ressources/{ressource_id}")]
pub async fn get_ressource(
    bastion_id: web::Path<i32>,
    ressource_id: web::Path<i32>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let _claims = Claims::verifier_session_admin(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;

    let ressource = Ressource::find_a_ressource(ressource_id.into_inner(),bastion_id.into_inner())?;
    Ok(HttpResponse::Ok().json(ressource))
}

#[delete("/bastions/{bastion_id}/ressources/{ressource_id}")]
pub async fn delete_a_ressource(
    bastion_id: web::Path<i32>,
    ressource_id: web::Path<i32>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let _claims = Claims::verifier_session_admin(&session)
        .ok_or(ApiError::new(404, "Not Found".to_string()))
        .map_err(|e| e)?;
    let ressource_id = ressource_id.into_inner();
    let bastion_id = bastion_id.into_inner();
    let ressource = Ressource::find_a_ressource(ressource_id,bastion_id)?;
    let ressource = Ressource::delete_a_ressource(ressource_id,bastion_id)?;
    Ok(HttpResponse::Ok().json(ressource))
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

    cfg.service(get_user_wireguard_status);
    /*
    cfg.service(find_server_config);
    cfg.service(update_server_config); */
}
