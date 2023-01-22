use actix_web::{Responder, web, post};
use crate::{BastionConfig, k8s};

#[post("/create/{bastion_id}")]
async fn create_bastion(bastion_config: web::Json<BastionConfig>, bastion_id: web::Path<i32>) -> impl Responder {
    match k8s::create_bastion(bastion_id.as_ref().to_owned(), &bastion_config.into_inner()).await {
        Ok(_) => format!("Bastion {}", bastion_id),
        Err(e) => format!("Error: {}", e),
    }
}

#[post("/delete/{bastion_id}")]
async fn delete_bastion(bastion_id: web::Path<i32>) -> impl Responder {
    match k8s::delete_bastion(bastion_id.as_ref().to_owned()).await {
        Ok(_) => format!("Bastion {} deleted", bastion_id),
        Err(e) => format!("Error: {}", e),
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_bastion)
        .service(delete_bastion);
}