use crate::consts::{CMD_IPTABLES, WG_PRIVATE_KEY_PATH};
use crate::{WGInterfaceConfig, WGPeerConfig, WGToAgent, WGToClient, WireguardAgent, WireguardRessource};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use log::error;

static COMMAND_IP: &str = "/sbin/ip";
static COMMAND_WG: &str = "/usr/bin/wg";

fn configure_ip_interface(interface: &str, ip: &str) -> Result<(), String> {
    let output = Command::new(COMMAND_IP)
        .arg("address")
        .arg("add")
        .arg(ip)
        .arg("dev")
        .arg(interface)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        error!("{}", String::from_utf8_lossy(&output.stdout));
        return Err("Can't set ip address".to_string())
    }
    Ok(())
}

fn up_interface(interface: &str) -> Result<(), String> {
    let output = Command::new(COMMAND_IP)
        .arg("link")
        .arg("set")
        .arg("mtu")
        .arg("1370")
        .arg("up")
        .arg("dev")
        .arg(interface)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        error!("{}", String::from_utf8_lossy(&output.stdout));
        return Err("Couldn't UP interface".to_string())
    }
    Ok(())
}

fn add_route(interface: &str, cidr_remote: &str) -> Result<(), String> {
    let output = Command::new(COMMAND_IP)
        .arg("-4")
        .arg("route")
        .arg("add")
        .arg(cidr_remote)
        .arg("dev")
        .arg(interface)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        error!("{}", String::from_utf8_lossy(&output.stdout));
        return Err("Couldn't add route".to_string())
    }
    Ok(())
}

fn configure_wg_interface(interface: &str, config: WGInterfaceConfig) -> Result<(), String> {
    let output: Output = if let Some(port) = config.listen_port {
        Command::new(COMMAND_WG)
            .arg("set")
            .arg(interface)
            .arg("private-key")
            .arg(config.private_key_path)
            .arg("listen-port")
            .arg(port.to_string())
            .output()
            .map_err(|e| e.to_string())?
    } else {
        Command::new(COMMAND_WG)
            .arg("set")
            .arg(interface)
            .arg("private-key")
            .arg(config.private_key_path)
            .output()
            .map_err(|e| e.to_string())?
    };

    if !output.status.success() {
        error!("{}", String::from_utf8_lossy(&output.stdout));
        return Err("Can't configure wireguard interface".to_string())
    }
    Ok(())
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
        error!("{}", String::from_utf8_lossy(&output.stdout));
        return Err("Can't add peer".to_string());
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

pub fn configure_to_agent(config: &WireguardAgent) -> Result<(), String> {
    let interface = "wg-agent";
    configure_ip_interface(interface, "10.10.1.2/24")?;
    up_interface(interface)?;
    add_route(interface, &config.target_cidr)?;
    let interface_config = WGInterfaceConfig {
        listen_port: None,
        private_key_path: WG_PRIVATE_KEY_PATH.to_string(),
    };
    configure_wg_interface(interface, interface_config)?;
    let peer = WGPeerConfig {
        public_key: config.public_key.to_string(),
        allowed_ips: config.target_cidr.to_string(),
        endpoint: Some(config.endpoint.to_string()),
    };
    add_peer(interface, &peer)?;
    Ok(())
}

pub fn configure_to_client(config: WGToClient, peers: Vec<WGPeerConfig>) {
    let interface = "wg-client";
    configure_ip_interface(interface, format!("10.10.{}.1/24", config.net_id).as_str()).unwrap();
    up_interface(interface).unwrap();
    let interface_config = WGInterfaceConfig {
        listen_port: Some(60244),
        private_key_path: config.private_key_path,
    };
    configure_wg_interface(interface, interface_config).unwrap();
    load_peers(interface, peers).unwrap();
}


fn set_target_ip(action: &str, ressource: &WireguardRessource) -> Result<(), String> {
    let output = Command::new(CMD_IPTABLES)
        .arg(action).arg("FORWARD")
        .arg("-i").arg("wg-client")
        .arg("-o").arg("wg-agent")
        .arg("-s").arg(&ressource.client_ip)
        .arg("-d").arg(&ressource.target_ip)
        .arg("-j").arg("ACCEPT")
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        return Err("Can't set target ip".to_string());
    }
    Ok(())
}

/// Allow traffic from wg-client to wg-agent with specific source ip and destination ip
///
/// `iptables -A FORWARD -i wg-client -o wg-agent -s <client_ip> -d <target_ip> -j ACCEPT`
pub fn allow_target_ip(ressource: &WireguardRessource) -> Result<(), String> {
    set_target_ip("-I", ressource)
}

/// Deny traffic from wg-client to wg-agent with specific source ip and destination ip
/// 
/// `iptables -D FORWARD -i wg-client -o wg-agent -s <client_ip> -d <target_ip> -j ACCEPT`
pub fn deny_target_ip(ressource: &WireguardRessource) -> Result<(), String> {
    set_target_ip("-D", ressource)
}