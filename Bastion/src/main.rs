use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use bastion_mania_bastion::wireguard::{persistance};
use bastion_mania_bastion::{api, init};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    init::startup();
    persistance::init_peers().expect("Erreur création fichier persistance !");

    HttpServer::new(|| App::new().configure(api::config).wrap(Logger::default()))
        .bind(("0.0.0.0", 9000))?
        .run()
        .await
}
