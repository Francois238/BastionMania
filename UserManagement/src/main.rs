use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use simple_logger::SimpleLogger;

mod schema;
mod tools;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    SimpleLogger::new().env().init().unwrap();

    HttpServer::new(|| App::new().configure(user::routes_user_utilisation))
        .bind(("0.0.0.0", 8082))?
        .run()
        .await
}
