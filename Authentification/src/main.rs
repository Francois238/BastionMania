#[macro_use]
extern crate log;

use dotenvy::dotenv;
use actix_web::{ App, HttpServer};

mod tools;
mod schema;
mod admin;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();


    HttpServer::new(|| {
        App::new()
            .configure(admin::routes_admin)
            .configure(user::routes_user)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}