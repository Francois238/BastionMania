use std::fs;
use std::process::Command;

use crate::consts::*;
use crate::ssh::model::SSHRessource;

pub fn add_ressource(ressource: SSHRessource) -> Result<(), String> {
    add_user(&ressource.name)?;
    unlock_user(&ressource.name)?;
    create_ssh_dir(&ressource.name)?;
    create_authorized_keys_file(&ressource.name)?;
    save_ressource(ressource)?;
    Ok(())
}

fn add_user(name: &str) -> Result<(), String> {
    let output = Command::new(CMD_USERADD)
        .arg("-D")
        .arg(name)
        .output()
        .map_err(|e| format!("Error adding user: {}", e))?;
    output.status.success()
        .then(|| ())
        .ok_or_else(|| format!("Error adding user: {}", String::from_utf8_lossy(&output.stderr)))?;
    Ok(())
}

fn unlock_user(name: &str) -> Result<(), String> {
    let output = Command::new(CMD_PASSWD)
        .arg("-u")
        .arg(name)
        .output()
        .map_err(|e| format!("Error unlocking user: {}", e))?;
    output.status.success()
        .then(|| ())
        .ok_or_else(|| format!("Error unlocking user: {}", String::from_utf8_lossy(&output.stderr)))?;
    Ok(())
}

fn create_ssh_dir(name: &str) -> Result<(), String> {
    fs::create_dir(format!("/home/{}/.ssh", name))
        .map_err(|e| format!("Error creating ssh dir: {}", e))?;
    Ok(())
}

fn create_authorized_keys_file(name: &str) -> Result<(), String> {
    fs::File::create(format!("/home/{}/.ssh/authorized_keys", name))
        .map_err(|e| format!("Error creating authorized_keys file: {}", e))?;
    Ok(())
}

fn save_ressource(ressource: SSHRessource) -> Result<(), String> {
    let ressource_json = serde_json::to_string(&ressource)
        .map_err(|e| format!("Error serializing ressource: {}", e))?;
    fs::write(format!("/home/{}/.ressource", ressource.name), ressource_json)
        .map_err(|e| format!("Error saving ressource: {}", e))?;
    Ok(())
}

fn load_ressource(name: &str) -> Result<SSHRessource, String> {
    let ressource_json = fs::read_to_string(format!("/home/{}/.ressource", name))
        .map_err(|e| format!("Error loading ressource: {}", e))?;
    serde_json::from_str(&ressource_json)
        .map_err(|e| format!("Error deserializing ressource: {}", e))
}