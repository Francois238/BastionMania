use actix_web::web::Data;
use actix_web::{App, HttpServer};
use bastion_mania_bastioninstancieur::{api, InstancieurConfig};
use kube::Client;
use simple_logger::SimpleLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    let client = Client::try_default().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .configure(api::config)
            .app_data(Data::new(InstancieurConfig::new(client.clone()).unwrap()))
    })
    .bind(("0.0.0.0", 9000))?
    .run()
    .await
}
