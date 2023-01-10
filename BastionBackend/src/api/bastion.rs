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
use derive_more::{Display};
use serde::{Serialize, Deserialize};
use serde_json::json;



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

#[get("/bastion")]
pub async fn get_bastion() -> Result<HttpResponse, ApiError>{
    let bastion_insere = find_all()?;


 
        Ok(HttpResponse::Ok().json(bastion_insere))
}

#[post("/bastion")]
pub async fn create_bastion( bastion: web::Json<BastionCreation>) -> Result<HttpResponse, ApiError>{

    let bastion_insere = create(bastion.into_inner())?;


 
        Ok(HttpResponse::Ok().json(bastion_insere))
        

    
}





#[get("/bastion/{bastion_id}")]
pub async fn find_a_bastion(bastion_id:  web::Path<i32>) -> Result<HttpResponse, ApiError>{
    let bastion_affiche = find_un_bastion(bastion_id.into_inner())?;
    Ok(HttpResponse::Ok().json(bastion_affiche))

}

#[patch("/bastion/{bastion_id}")]
pub async fn update_a_bastion(bastion_id:web::Path<i32>, modifs:web::Json<BastionModification>) -> Result<HttpResponse, ApiError>{
    let bastion_modif = update_un_bastion(bastion_id.into_inner(), modifs.into_inner())?;
    Ok(HttpResponse::Ok().json(bastion_modif))
}

#[delete("/bastion/{bastion_id}")]
pub async fn delete_a_bastion(bastion_id:web::Path<i32>) -> Result<HttpResponse, ApiError>{
    let bastion_suppr = delete_un_bastion(bastion_id.into_inner())?;
    Ok(HttpResponse::Ok().json("supprimé"))
}






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






#[get("/bastion/{bastion_id}/users")]
pub async fn get_users() -> impl Responder{
    HttpResponse::Ok().body("liste_users")
}

#[post("/bastion/{bastion_id}/users")]
pub async fn create_users() -> impl Responder{
    HttpResponse::Ok().body("création_droits_accés")
}


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
    cfg.service(find_server_config); 
    cfg.service(update_server_config); 
}
    
   

