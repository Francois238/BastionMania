use actix_web::{App, HttpServer};
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
            .configure(admin::routes_admin)
            .configure(user::routes_user)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
