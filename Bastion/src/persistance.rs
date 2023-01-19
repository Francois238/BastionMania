use std::fs;
use std::path::Path;
use crate::WGPeerConfig;

static PEERS_PATH: &str = "/peers/";
static PEERS_FILE: &str = "peers.json";


pub fn init_peers() -> Result<(), String> {
    let peers_files_path = Path::new(PEERS_PATH);
    if !peers_files_path.exists() {
        fs::create_dir_all(PEERS_PATH)
            .map_err(|e| format!("Error creating peers directory: {}", e))?;
    }
    let peers_files_path = format!("{}{}", PEERS_PATH, PEERS_FILE);
    let peers_files_path = Path::new(peers_files_path.as_str());
    if peers_files_path.exists() {
        println!("Peers file already exists");
        return Ok(());
    }
    println!("Peers file does not exist, creating it");

    fs::write(format!("{}{}", PEERS_PATH, PEERS_FILE), "[]")
        .map_err(|e| format!("Error creating peers file: {}", e))?;

    Ok(())
}

pub fn get_peers() -> Result<Vec<WGPeerConfig>, String> {
    let peers = fs::read_to_string(format!("{}{}", PEERS_PATH, PEERS_FILE))
        .map_err(|e| format!("Error reading peers file: {}", e))?;

    Ok(serde_json::from_str(&peers)
        .map_err(|e| format!("Error parsing peers file: {}", e))?)
}

pub fn save_peers(peers: Vec<WGPeerConfig>) -> Result<(), String> {
    let peers = serde_json::to_string(&peers)
        .map_err(|e| format!("Error converting peers to string : {}", e))?;

    fs::write(format!("{}{}", PEERS_PATH, PEERS_FILE), peers)
        .map_err(|e| format!("Error writing peers file: {}", e))?;

    Ok(())
}

pub fn add_peer(peer: WGPeerConfig) -> Result<(), String> {
    let mut peers = get_peers()?;

    peers.push(peer);

    save_peers(peers)?;

    Ok(())
}

pub fn remove_peer(public_key: String) -> Result<(), String> {
    let mut peers = get_peers()?;

    peers.retain(|p| p.public_key != public_key);

    save_peers(peers)?;

    Ok(())
}
