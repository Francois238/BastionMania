use std::fmt::format;
use std::fs;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::consts::*;
use crate::ssh::authorized_keys::{AuthorizedKey, AuthorizedKeys};
use crate::ssh::user::SSHUser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHRessource {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

fn add_system_user(name: &str) -> Result<(), String> {
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

fn unlock_system_user(name: &str) -> Result<(), String> {
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

fn save_ressource(ressource: &SSHRessource) -> Result<(), String> {
    let ressource_json = serde_json::to_string(&ressource)
        .map_err(|e| format!("Error serializing ressource: {}", e))?;
    fs::write(format!("/home/{}/.ressource", ressource.name), ressource_json)
        .map_err(|e| format!("Error saving ressource: {}", e))?;
    Ok(())
}

impl SSHRessource {
    pub fn save(&self) -> Result<(), String> {
        add_system_user(&self.name)?;
        unlock_system_user(&self.name)?;
        create_ssh_dir(&self.name)?;
        create_authorized_keys_file(&self.name)?;
        save_ressource(self)?;
        Ok(())
    }
    pub fn from_name(name: &str) -> Result<SSHRessource, String> {
        let ressource_json = fs::read_to_string(format!("/home/{}/.ressource", name))
            .map_err(|e| format!("Error loading ressource: {}", e))?;
        serde_json::from_str(&ressource_json)
            .map_err(|e| format!("Error deserializing ressource: {}", e))
    }

    pub fn add_user(&self, user: &SSHUser) -> Result<(), String> {
        let path = self.authorized_keys_path();
        let mut authorized_keys = AuthorizedKeys::from_path(path.as_str())?;
        let authorized_key = AuthorizedKey::new(self, user)?;
        authorized_keys.add_key(authorized_key);
        authorized_keys.save(path.as_str())?;
        Ok(())
    }
}

impl SSHRessource {
    fn authorized_keys_path(&self) -> String {
        format!("/home/{}/.ssh/authorized_keys", self.name)
    }
}
