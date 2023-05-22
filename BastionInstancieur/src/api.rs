use actix_web::{Responder, web, post, delete, HttpResponse};
use crate::{BastionConfig, InstancieurConfig};
use crate::bastions::BastionSpec;

/// Create a new bastion
///
/// Create a new ressource bastions.bastionmania.fr/v1alpha1 on the kubernetes cluster
#[post("/create")]
async fn create_bastion(bastion_config: web::Json<BastionConfig>, instancieur_config: web::Data<InstancieurConfig>) -> impl Responder {
    let bastion_config = bastion_config.into_inner();
    let instancieur_config = instancieur_config.get_ref();
    let bastion_spec = BastionSpec::new(bastion_config.clone(), &instancieur_config.image);
    match bastion_spec.create(instancieur_config.client.clone()).await {
        Ok(_) => log::info!("Bastion {} created", bastion_config.bastion_id),
        Err(e) => {
            log::error!("Error creating Bastion {}: {}",bastion_config.bastion_id, e);
            return HttpResponse::InternalServerError().body(format!("Error creating Bastion {}", bastion_config.bastion_id));
        }
    }

    HttpResponse::Ok().body(format!("Bastion {} created", bastion_config.bastion_id))
}

#[delete("/delete/{bastion_id}")]
async fn delete_bastion(bastion_id: web::Path<String>, instancieur_config: web::Data<InstancieurConfig>) -> impl Responder {
    let bastion_id = bastion_id.into_inner();
    log::info!("Deleting bastion {}", bastion_id);
    match BastionSpec::delete(&format!("bastion-{bastion_id}"), instancieur_config.client.clone()).await{
        Ok(_) => log::info!("Bastion {} deleted", bastion_id),
        Err(e) => {
            log::error!("Error deleting Bastion {}: {}",bastion_id, e);
            return HttpResponse::InternalServerError().body(format!("Error deleting Bastion {}", bastion_id));
        }
    }

    HttpResponse::Ok().body(format!("Bastion {} deleted", bastion_id))
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_bastion)
        .service(delete_bastion);
}