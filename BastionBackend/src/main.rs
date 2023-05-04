use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, App, HttpServer};
use dotenvy::dotenv;
use BastionManager::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    HttpServer::new(|| {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .configure(api::routes_bastion)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
