use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use BastionManager::api;
use actix_web::{App, HttpServer, cookie::Key};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();

    
    HttpServer::new(|| {
        App::new( )
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(),Key::from(&[0; 64])).cookie_secure(false).build()
            )
            .configure(api::routes_bastion)
            })
                .bind(("0.0.0.0", 8080))?
                .run()
                .await
}