use crate::{WGInterfaceConfig, WGPeerConfig, WGToAgent, WGToClient};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

static COMMAND_IP: &str = "/sbin/ip";
static COMMAND_WG: &str = "/usr/bin/wg";

fn configure_ip_interface(interface: &str, ip: &str) {
    let output = Command::new(COMMAND_IP)
        .arg("address")
        .arg("add")
        .arg(ip)
        .arg("dev")
        .arg(interface)
        .output()
        .expect("Failed to execute");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        panic!("Can't set ip address");
    }
}

fn up_interface(interface: &str) {
    let output = Command::new(COMMAND_IP)
        .arg("link")
        .arg("set")
        .arg("mtu")
        .arg("1370")
        .arg("up")
        .arg("dev")
        .arg(interface)
        .output()
        .expect("Failed to execute");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        panic!("Couldn't UP interface");
    }
}

fn add_route(interface: &str, cidr_remote: &str) {
    let output = Command::new(COMMAND_IP)
        .arg("-4")
        .arg("route")
        .arg("add")
        .arg(cidr_remote)
        .arg("dev")
        .arg(interface)
        .output()
        .expect("Failed to execute");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        panic!("Couldn't add route");
    }
}

fn configure_wg_interface(interface: &str, config: WGInterfaceConfig) {
    let output: Output = if let Some(port) = config.listen_port {
        Command::new(COMMAND_WG)
            .arg("set")
            .arg(interface)
            .arg("private-key")
            .arg(config.private_key_path)
            .arg("listen-port")
            .arg(port.to_string())
            .output()
            .expect("Failed to execute")
    } else {
        Command::new(COMMAND_WG)
            .arg("set")
            .arg(interface)
            .arg("private-key")
            .arg(config.private_key_path)
            .output()
            .expect("Failed to execute")
    };

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        panic!("Can't configure wireguard interface");
    }
}

fn load_peers(interface: &str, peers: Vec<WGPeerConfig>) -> Result<(), String> {
    for peer in peers {
        add_peer(interface, &peer)?;
    }
    Ok(())
}

pub fn add_peer(interface: &str, peer: &WGPeerConfig) -> Result<(), String> {
    let peer = peer.to_owned();
    let output = if let Some(endpoint) = peer.endpoint {
        Command::new(COMMAND_WG)
            .arg("set")
            .arg(interface)
            .arg("peer")
            .arg(peer.public_key)
            .arg("allowed-ips")
            .arg(peer.allowed_ips)
            .arg("endpoint")
            .arg(endpoint)
            .output()
            .map_err(|e| e.to_string())?
    } else {
        Command::new(COMMAND_WG)
            .arg("set")
            .arg(interface)
            .arg("peer")
            .arg(peer.public_key)
            .arg("allowed-ips")
            .arg(peer.allowed_ips)
            .output()
            .map_err(|e| e.to_string())?
    };
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        return Err("Can't load peer".to_string());
    }
    Ok(())
}

pub fn remove_peer(interface: &str, peer_public_key: &str) -> Result<(), String> {
    let output = Command::new(COMMAND_WG)
        .arg("set")
        .arg(interface)
        .arg("peer")
        .arg(peer_public_key)
        .arg("remove")
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        return Err("Can't remove peer".to_string());
    }
    Ok(())
}

// Ecrit la clé dans un fichier et renvoie le chemin du fichier
pub fn write_key_to_file(name: &str, keytype: &str, value: &str) -> Result<String, String> {
    let exist = Path::new("/keys").exists();
    if !exist {
        println!("Creating /keys");
        fs::create_dir("/keys").expect("Can't create /keys");
    }

    let path = format!("/keys/{}-{}", keytype, name);
    fs::write(&path, value).map_err(|e| e.to_string())?;
    Ok(path)
}

pub fn configure_to_agent(config: WGToAgent) {
    let interface = "wg-agent";
    configure_ip_interface(interface, "10.10.1.2/24");
    up_interface(interface);
    add_route(interface, &config.net_cidr);
    let interface_config = WGInterfaceConfig {
        listen_port: None,
        private_key_path: config.private_key_path,
    };
    configure_wg_interface(interface, interface_config);
    let peer = WGPeerConfig {
        public_key: config.agent_public_key,
        allowed_ips: config.net_cidr,
        endpoint: Some(config.agent_endpoint),
    };
    add_peer(interface, &peer).unwrap();
}

pub fn configure_to_client(config: WGToClient, peers: Vec<WGPeerConfig>) {
    let interface = "wg-client";
    configure_ip_interface(interface, format!("10.10.{}.1/24", config.net_id).as_str());
    up_interface(interface);
    let interface_config = WGInterfaceConfig {
        listen_port: Some(60244),
        private_key_path: config.private_key_path,
    };
    configure_wg_interface(interface, interface_config);
    load_peers(interface, peers).unwrap();
}
