use std::borrow::BorrowMut;
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
use crate::services::{generate_freenetid, generate_freeport};
//use derive_more::{Display};
use serde::{Serialize, Deserialize};
use serde_json::json;
use repository::*;
use crate::entities::{Bastion, BastionInsertable};
use crate::model::{BastionInstanceCreate, BastionModification, ConfigAgent, RetourAPI};


/*
#[derive(Debug, Display)]
pub enum TaskError {
    NonAuthentifie,
    NonAutorise,
    Inexistant

}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::NonAuthentifie => StatusCode::UNAUTHORIZED,
            TaskError::NonAutorise => StatusCode::FORBIDDEN,
            TaskError::Inexistant => StatusCode::NOT_FOUND,

        }
    }
}
*/

// /bastion =======================================================================================

#[get("/bastion")]
pub async fn get_bastion() -> Result<HttpResponse, ApiError>{
    let bastion_insere = Bastion::find_all()?;
    Ok(HttpResponse::Ok().json(bastion_insere))
}

#[post("/bastion")]
pub async fn create_bastion( bastion: web::Json<BastionModification>) -> Result<HttpResponse, ApiError>{

    //generation pair de clé pour agent et bastion
    let agent_priv = wireguard_keys::Privkey::generate();
    let agent_pub = agent_priv.pubkey();
    let bastion_priv = wireguard_keys::Privkey::generate();
    let bastion_pub = bastion_priv.pubkey();

    let liste_bastions=Bastion::find_all()?;
    //generation d'un port libre pour le bastion
    let ports: Vec<i32> = (&liste_bastions).into_iter().map(|b| b.port ).collect();
    let port = generate_freeport(&ports);
    //generation d'un net_id libre pour le bastion
    let net_ids: Vec<i32> = liste_bastions.into_iter().map(|b| b.net_id ).collect();
    let net_id = generate_freenetid(&net_ids);

    //creation du bastion
    let bastion_insertion: BastionInsertable = BastionInsertable{
        name: bastion.name.clone(),
        subnet_cidr: bastion.subnet_cidr.clone(),
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
pub async fn find_a_bastion(bastion_id:  web::Path<i32>) -> Result<HttpResponse, ApiError>{
    let bastion_affiche = Bastion::find_un_bastion(bastion_id.into_inner())?;
    Ok(HttpResponse::Ok().json(bastion_affiche))

}

#[patch("/bastion/{bastion_id}")]
pub async fn update_a_bastion(bastion_id:web::Path<i32>, modifs:web::Json<BastionModification>) -> Result<HttpResponse, ApiError>{
    let bastion_modif = Bastion::update_un_bastion(bastion_id.into_inner(), modifs.into_inner())?;
    Ok(HttpResponse::Ok().json(bastion_modif))
}

#[delete("/bastion/{bastion_id}")]
pub async fn delete_a_bastion(bastion_id:web::Path<i32>) -> Result<HttpResponse, ApiError>{
    let bastion_suppr = Bastion::delete_un_bastion(bastion_id.into_inner())?;
    Ok(HttpResponse::Ok().json("supprimé"))
}

// /bastion/{bastion_id}/wireguard ================================================================
/*
#[get("/bastion/{bastion_id}/wireguard")]
pub async fn find_server_config(bastion_id:  web::Path<i32>) -> Result<HttpResponse, ApiError>{

    let bastion = find_un_bastion(bastion_id.into_inner())?;
    match bastion.wireguard_id{
        Some(value) => {
            let serveur_conf = get_server_config(value)?;
            Ok(HttpResponse::Ok().json(serveur_conf))

        }

        None => Err(ApiError::new(400, "pas de bastion".to_string()))
    }
}

#[patch("/bastion/{bastion_id}/wireguard")]
pub async fn update_server_config(bastion_id:web::Path<i32>, modifs:web::Json<WireguardServerModification>) -> Result<HttpResponse, ApiError>{

    let bastion = find_un_bastion(bastion_id.into_inner())?;
    match bastion.wireguard_id{
        Some(value) =>{
            let server_modif = patch_server_config(value, modifs.into_inner())?;
            Ok(HttpResponse::Ok().json(server_modif))            
        }
        None => Err(ApiError { status_code: (400), message: ("pas de bastion".to_string()) })
    }
}
*/
//  /bastion/{bastion_id}/users ===================================================================

#[get("/bastion/{bastion_id}/users")]
pub async fn get_users() -> impl Responder{
    HttpResponse::Ok().body("liste_users")
}

#[post("/bastion/{bastion_id}/users")]
pub async fn create_users() -> impl Responder{
    HttpResponse::Ok().body("création_droits_accés")
}

// /bastion/{bastion_id}/users/{user_id} =========================================================

#[get("/bastion/{bastion_id}/users/{user_id}")]
pub async fn get_a_user() -> impl Responder{
    HttpResponse::Ok().body("user_x")
}

#[patch("/bastion/{bastion_id}/users/{user_id}")]
pub async fn modify_a_user() -> impl Responder{
    HttpResponse::Ok().body("modification_user_x")
}

#[delete("/bastion/{bastion_id}/users/{user_id}")]
pub async fn delete_a_user() -> impl Responder{
    HttpResponse::Ok().body("delete_user_x")
}

// /bastion/{bastion_id}/users/{user_id}/wireguard" ===============================================

#[get("/bastion/{bastion_id}/users/{user_id}/wireguard")]
pub async fn get_user_wireguard() -> impl Responder{
    HttpResponse::Ok().body("wireguard_user_x_info")
}

#[patch("/bastion/{bastion_id}/users/{user_id}/wireguard")]
pub async fn modify_user_wireguard() -> impl Responder{
    HttpResponse::Ok().body("wireguard_user_x_modified")
}

#[delete("/bastion/{bastion_id}/users/{user_id}/wireguard")]
pub async fn delete_user_wireguard() -> impl Responder{
    HttpResponse::Ok().body("wireguard_user_x_severed")
}

pub fn routes_bastion(cfg: &mut web::ServiceConfig) {
    cfg.service(create_bastion);
    cfg.service(get_bastion);

    cfg.service(update_a_bastion);
    cfg.service(find_a_bastion);
    cfg.service(delete_a_bastion);
/*
    cfg.service(find_server_config); 
    cfg.service(update_server_config); */
}
    
   

