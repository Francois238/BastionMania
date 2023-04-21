use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use simple_logger::SimpleLogger;

mod admin;
mod schema;
mod tools;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    SimpleLogger::new().env().init().unwrap();

    HttpServer::new(|| App::new().configure(admin::routes_admin_utilisation))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}
