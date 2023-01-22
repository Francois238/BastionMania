use actix_web::{App, HttpServer};
use bastion_mania_bastioninstancieur::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();


    // Wait for Pod to be running
    // loop{
    //     let bastion_pod_recup = pods.get(&format!("bastion-{}", bastion_id)).await.map_err(|e| e.to_string())?;
    //     let phase = bastion_pod_recup.status.unwrap().phase.unwrap();
    //     info!("Pod bastion-{} phase: {}", bastion_id, phase);
    //     if phase != "Pending" {
    //         break;
    //     }
    //     thread::sleep(std::time::Duration::from_secs(1));
    // }


    HttpServer::new(|| {
        App::new().configure(api::config)
    })
        .bind(("0.0.0.0", 9000))?
        .run()
        .await
}