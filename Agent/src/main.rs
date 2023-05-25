use std::{env, fs, thread, time};
use std::process::Command;
use log::{LevelFilter};
use reqwest::{Certificate};
use reqwest::blocking::ClientBuilder;
use simple_logger::SimpleLogger;
use wireguard_keys::Privkey;
use bastion_mania_agent::wireguard::WireguardConf;

fn generate_private_key(){
    let priv_key = Privkey::generate();
    fs::write("/data/private_key", priv_key.to_base64()).expect("Unable to write file");
}
fn get_public_key() -> String {
    let priv_key = fs::read_to_string("/data/private_key").expect("Unable to read file");
    let priv_key = Privkey::from_base64(&priv_key).expect("Invalid private key");
    priv_key.pubkey().to_base64()
}
fn ensure_private_key(){
    if !fs::metadata("/data/private_key").is_ok() {
        log::info!("Private key not found, generating one");
        generate_private_key();
    }
}

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).env().init().unwrap();

    log::info!("BastionMania Agent starting");

    ensure_private_key();

    if !fs::metadata("/data/bastion_public_key").is_ok() {
        // Use token to initiate bastion pairing
        let config = bastion_mania_agent::LaunchConfig::new().expect("Unable to read config");
        let pair_config = bastion_mania_agent::PairConfig {
            token: config.token,
            public_key: get_public_key(),
            agent_host: config.agent_host,
        };
        let certificate = fs::read("/certs/bastionmania.intra.pem").expect("Unable to read certificate");
        let certificate = Certificate::from_pem(&certificate).expect("Unable to parse certificate");
        let client = ClientBuilder::new()
            .tls_built_in_root_certs(false)
            .add_root_certificate(certificate)
            .build().expect("Unable to build client");
        log::info!("Initiating pairing with bastion");
        let res = client.post(&format!("https://{}/api/agent", config.bm_host))
            .json(&pair_config)
            .send()
            .expect("Unable to send request");
        if !res.status().is_success() {
            log::error!("Pairing failed: {}", res.text().expect("Unable to read response"));
            panic!();
        }
        log::info!("Pairing successful, saving bastion public key");
        let bastion_public_key = res.text().expect("Unable to read response");
        fs::write("/data/bastion_public_key", bastion_public_key).expect("Unable to write file");
    }

    let config = WireguardConf {
        priv_key: fs::read_to_string("/data/private_key").expect("Unable to read file"),
        peer_public: fs::read_to_string("/data/bastion_public_key").expect("Unable to read file"),
    };

    fs::write("/etc/wireguard/wg0.conf", config.to_string()).expect("Unable to write file");

    let out = Command::new("/usr/bin/wg-quick")
        .arg("up")
        .arg("wg0")
        .output()
        .expect("failed to execute process");

    if !out.status.success() {
        log::error!("stdout: {}", String::from_utf8_lossy(&out.stdout));
        log::error!("Error: {}", String::from_utf8_lossy(&out.stderr));
        panic!();
    }

    println!("Wireguard server started");

    // infinite sleep
    loop {
        thread::sleep(time::Duration::from_secs(1000));
    }
}
