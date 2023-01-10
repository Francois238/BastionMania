#[macro_use]
extern crate log;

use dotenvy::dotenv;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{ App, HttpServer, cookie::Key};

mod api_error;
mod db;
mod schema;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            .configure(user::routes_user_utilisation)
    })
    .bind(("0.0.0.0", 8082))?
    .run()
    .await
}