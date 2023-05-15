use std::fs;
use std::process::Command;
use crate::consts::CMD_SSHKEYGEN;

use log::info;

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
    let private_key_path = "/data/wg_private_key";
    fs::write(private_key_path, private_key.to_base64()).expect("Failed to write private key");

    // Generate init proof
    fs::File::create("/data/first_init").expect("Failed to create first_init file");
    fs::File::create("/bastion_init").expect("Failed to create bastion_init file");
}

fn recreate_init(){
    info!("Recreate init");
    // copy key from /data
    info!("Copying ssh host keys from /data");
    Command::new("cp")
        .arg("/data/ssh_keys/ssh_host_*")
        .arg("/etc/ssh/")
        .output()
        .expect("Failed to copy ssh host keys");

    // TODO: restore ssh ressources
}