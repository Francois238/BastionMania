use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{App, HttpServer, cookie::{Key, SameSite}};
use dotenvy::dotenv;
use simple_logger::SimpleLogger;

mod admin;
mod schema;
mod tools;
mod user;

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
                .cookie_same_site(SameSite::None)
                .build()
        )
            .configure(admin::routes_admin)
            .configure(user::routes_user)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
