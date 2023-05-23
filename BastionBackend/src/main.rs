use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use BastionManager::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    

    HttpServer::new(|| {
        let scope = web::scope("/api").configure(api::routes_bastion);
        App::new().service(scope)
            
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
