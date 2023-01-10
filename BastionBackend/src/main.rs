use BastionManager::api;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();

    
    HttpServer::new(|| App::new()
    .configure(api::routes_bastion))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}