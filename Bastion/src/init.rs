use std::fs;
use std::path::Path;
use std::process::Command;

use log::info;

use crate::{BastionConfig, WGToClient};
use crate::consts::{CMD_SSHKEYGEN, WG_PRIVATE_KEY_PATH};
use crate::database::BastionDatabase;
use crate::wireguard::{self, wgconfigure};


const SSH_HOST_KEYS_FILES: [&str; 6] = [
    "ssh_host_ecdsa_key",
    "ssh_host_ecdsa_key.pub",
    "ssh_host_ed25519_key",
    "ssh_host_ed25519_key.pub",
    "ssh_host_rsa_key",
    "ssh_host_rsa_key.pub",
];
const SSH_HOST_KEYS_PATH: &str = "/etc/ssh";
const SSH_HOST_KEYS_PATH_DATA: &str = "/data/ssh_keys";

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
    save_host_keys();

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
    restore_host_keys();

    info!("Restoring ssh resssources");
    let database = BastionDatabase::get().expect("Can't load database");
    let ssh_ressources = database.get_ssh_ressources();
    for ressource in ssh_ressources {
        ressource.realise().expect("Can't realise ressource");
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

fn save_host_keys() {
    copy_list(
        SSH_HOST_KEYS_PATH,
        SSH_HOST_KEYS_PATH_DATA,
        &SSH_HOST_KEYS_FILES,
    )
}

fn restore_host_keys() {
    copy_list(
        SSH_HOST_KEYS_PATH_DATA,
        SSH_HOST_KEYS_PATH,
        &SSH_HOST_KEYS_FILES,
    )
}

fn copy_list(src: &str, dst: &str, list: &[&str]) {
    for item in list {
        let src_path = format!("{}/{}", src, item);
        let dst_path = format!("{}/{}", dst, item);
        if Path::new(&src_path).exists() {
            fs::copy(src_path, dst_path).expect("Failed to copy ssh host keys");
        }
    }
}

fn init_proof_make() {
    fs::File::create("/bastion_init").expect("Failed to create bastion_init file");
}

fn init_proof_exists() -> bool {
    Path::new("/bastion_init").exists()
}

/// Start sshd and rsyslogd
fn start_sshd() {
    Command::new("/usr/sbin/rsyslogd")
        .output()
        .expect("Failed to start rsyslogd");

    Command::new("/usr/sbin/sshd")
        .output()
        .expect("Failed to start sshd");
}

fn init_wg() {
    let database = BastionDatabase::get().expect("Can't load database");
    let bastion_config = BastionConfig::new();

    if let Some(agent) = database.get_agent() {
        info!("Starting agent connection");
        wgconfigure::configure_to_agent(agent).expect("Can't configure agent connexion");
    }else {
        info!("No agent configured, skipping");
    }

    let config_to_client = WGToClient {
        private_key_path: WG_PRIVATE_KEY_PATH.to_string(),
        net_id: bastion_config.net_id,
    };

    let ressources = database.get_wireguard_ressources();
    for ressource in ressources {
        ressource.create().expect("Can't create wireguard config");
    }

    wgconfigure::configure_to_client(config_to_client, vec![]);
}
