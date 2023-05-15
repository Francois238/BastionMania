use crate::consts::CMD_SSHKEYGEN;
use crate::database::{self, BastionDatabase};
use crate::wireguard::{wgconfigure, self};
use crate::{BastionConfig, WGToAgent, WGToClient};
use std::fs;
use std::path::Path;
use std::process::Command;

use log::info;

const WG_PRIVATE_KEY_PATH: &str = "/data/wg_private_key";

/// Initialize the bastion when launch for the first time
///
/// This create ssh host keys and wireguard private key
/// It also initialize the database
fn first_init() {
    info!("First init");
    // Generate ssh host keys
    info!("Generating ssh host keys");
    Command::new(CMD_SSHKEYGEN)
        .arg("-A")
        .output()
        .expect("Failed to execute ssh-keygen");
    // copy keys to /data in case of container recreate
    info!("Copying ssh host keys to /data");
    fs::create_dir("/data/ssh_keys").expect("Failed to create /data/ssh_keys");
    Command::new("cp")
        .arg("/etc/ssh/ssh_host_*")
        .arg("/data/ssh_keys/")
        .output()
        .expect("Failed to copy ssh host keys");

    // Generate wireguard key
    info!("Generating wireguard keys");
    let private_key = wireguard_keys::Privkey::generate();
    fs::write(WG_PRIVATE_KEY_PATH, private_key.to_base64()).expect("Failed to write private key");

    // Generate init proof
    init_proof_make();

    // Initialize database
    info!("Initializing database");
    let database = BastionDatabase::get().expect("Faild to load database");
    database.save().expect("Failed to save database");
}

/// Initialize the bastion when the pod is rescheduled or recreated
///
/// This copy the ssh host keys from /data to /etc/ssh
/// It also restore ssh ressources and users
fn recreate_init() {
    info!("Recreate init");
    // copy key from /data
    info!("Copying ssh host keys from /data");
    Command::new("cp")
        .arg("/data/ssh_keys/ssh_host_*")
        .arg("/etc/ssh/")
        .output()
        .expect("Failed to copy ssh host keys");

    info!("Restoring ssh resssources");
    let database = BastionDatabase::get().expect("Can't load database");
    let ssh_ressources = database.get_ssh_ressources();
    for ressource in ssh_ressources {
        ressource.save().expect("Can't save ressource");
        ressource.add_all_users().expect("Can't add all users");
    }
}

/// Initialize the bastion when the pod is restarted
///
/// This function do the following
/// - Start sshd
/// - Start wireguard to agent if configured
/// - Start wireguard to client
/// - restore all Wireguard users
fn restart_init() {
    info!("Restart init");

    info!("Starting sshd");
    start_sshd();

    info!("Initializing wireguard");
    wireguard::init();
    init_wg();
}

pub fn startup() {
    if !BastionDatabase::exists() {
        first_init();
    }

    if !init_proof_exists() {
        recreate_init();
    }

    restart_init();
}

fn init_proof_make() {
    fs::File::create("/bastion_init").expect("Failed to create bastion_init file");
}

fn init_proof_exists() -> bool {
    Path::new("/bastion_init").exists()
}

fn start_sshd() {
    Command::new("/usr/sbin/sshd")
        .output()
        .expect("Failed to start sshd");
}

fn init_wg() {
    let bastion_config = BastionConfig::new();
    let config_to_agent = WGToAgent {
        agent_endpoint: bastion_config.agent_endpoint,
        agent_public_key: bastion_config.agent_public_key,
        private_key_path: WG_PRIVATE_KEY_PATH.to_string(),
        net_cidr: bastion_config.net_cidr,
    };

    let config_to_client = WGToClient {
        private_key_path: WG_PRIVATE_KEY_PATH.to_string(),
        net_id: bastion_config.net_id,
    };

    let database = BastionDatabase::get().expect("Can't load database");
    let peers = database
        .get_wireguard_ressources()
        .iter()
        .map(|r| r.to_wg_peer_config())
        .collect();

    wgconfigure::configure_to_agent(config_to_agent);
    wgconfigure::configure_to_client(config_to_client, peers);
}
