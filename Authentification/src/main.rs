use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key},
    App, HttpServer, middleware::Logger,
};
use dotenvy::dotenv;
use simple_logger::SimpleLogger;

mod admin;
mod schema;
mod tools;
mod user;
mod verification;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    SimpleLogger::new().env().init().unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(true)
                    .build(),
            )
            .wrap(
                Logger::default()
            )
            .configure(admin::routes_admin)
            .configure(user::routes_user)
            .configure(verification::routes_jwt_verif)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
