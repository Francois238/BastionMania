use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use BastionManager::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    HttpServer::new(|| {
        App::new()
            .configure(api::routes_bastion)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
