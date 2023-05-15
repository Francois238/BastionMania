use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use bastion_mania_bastion::startup::startup;
use bastion_mania_bastion::wireguard::{persistance, wgconfigure};
use bastion_mania_bastion::{api, BastionConfig, WGToAgent, WGToClient};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bastion_config = BastionConfig::new();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    startup();
    persistance::init_peers().expect("Erreur création fichier persistance !");

    let private_key_path =
        wgconfigure::write_key_to_file("bastion", "private", &bastion_config.bastion_private_key)
            .unwrap();

    let config_to_agent = WGToAgent {
        agent_endpoint: bastion_config.agent_endpoint,
        agent_public_key: bastion_config.agent_public_key,
        private_key_path: private_key_path.clone(),
        net_cidr: bastion_config.net_cidr,
    };

    let config_to_client = WGToClient {
        private_key_path: private_key_path.clone(),
        net_id: bastion_config.net_id,
    };

    let peers = persistance::get_peers().unwrap();

    wgconfigure::configure_to_agent(config_to_agent);
    wgconfigure::configure_to_client(config_to_client, peers);

    HttpServer::new(|| App::new().configure(api::config).wrap(Logger::default()))
        .bind(("0.0.0.0", 9000))?
        .run()
        .await
}
