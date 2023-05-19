use kube::{client::Client};

use bastion_kubernetes_ressource::controller;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // Create a Kubernetes client
    let client = Client::try_default().await?;

    loop {
        let res = controller::watch_bastion(client.clone()).await;
        match res {
            Ok(_) => {}
            Err(e) => {
                log::error!("Error: {}", e);
            }
        }
    }

}
