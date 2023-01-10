#[macro_use]
extern crate log;

use dotenvy::dotenv;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{ App, HttpServer, cookie::Key};

mod api_error;
mod db;
mod schema;
mod admin;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            .configure(admin::routes_admin_utilisation)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}