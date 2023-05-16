use log::debug;
use std::fs;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::consts::*;
use crate::database::BastionDatabase;
use crate::ssh::authorized_keys::{AuthorizedKey, AuthorizedKeys};
use crate::ssh::user::SSHUser;
use crate::ssh::utils::kill_all_sessions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHRessource {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub users: Vec<SSHUser>,
}

fn add_system_user(name: &str) -> Result<(), String> {
    let output = Command::new(CMD_USERADD)
        .arg("-D")
        .arg(name)
        .output()
        .map_err(|e| format!("Error adding user: {}", e))?;
    output.status.success().then_some(()).ok_or_else(|| {
        format!(
            "Error adding user: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    })?;
    Ok(())
}

fn unlock_system_user(name: &str) -> Result<(), String> {
    let output = Command::new(CMD_PASSWD)
        .arg("-u")
        .arg(name)
        .output()
        .map_err(|e| format!("Error unlocking user: {}", e))?;
    output.status.success().then_some(()).ok_or_else(|| {
        format!(
            "Error unlocking user: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    })?;
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

impl SSHRessource {
    pub fn save(&self) -> Result<(), String> {
        let mut database =
            BastionDatabase::get().map_err(|e| format!("Error loading database: {}", e))?;
        database
            .add_ssh(self.clone())
            .map_err(|e| format!("Error saving ressource: {}", e))?;
        Ok(())
    }

    pub fn realise(&self) -> Result<(), String> {
        add_system_user(&self.name)?;
        unlock_system_user(&self.name)?;
        create_ssh_dir(&self.name)?;
        create_authorized_keys_file(&self.name)?;
        Ok(())
    }
    pub fn from_name(name: &str) -> Result<SSHRessource, String> {
        let database =
            BastionDatabase::get().map_err(|e| format!("Error loading database: {}", e))?;
        let ressource = database
            .get_ssh_by_name(name)
            .ok_or_else(|| format!("Ressource {} not found", name))?;
        Ok(ressource.clone())
    }

    /// Add the user to the authorized_keys file and save it to database
    pub fn add_user(&self, user: &SSHUser) -> Result<(), String> {
        let mut database =
            BastionDatabase::get().map_err(|e| format!("Error loading database: {}", e))?;
        let path = self.authorized_keys_path();
        let mut authorized_keys = AuthorizedKeys::from_path(path.as_str())?;
        let authorized_key = AuthorizedKey::new(self, user)?;
        authorized_keys.add_key(authorized_key);
        debug!("authorized_keys: {:?}", authorized_keys);
        authorized_keys.save(path.as_str())?;

        let mut _self = database
            .get_ssh_mut_by_name(self.name.as_str())
            .ok_or(format!("Ressource {} not found", self.name))?;
        _self.users.push(user.clone());
        database
            .save()
            .map_err(|e| format!("Error saving database: {}", e))?;
        Ok(())
    }

    pub fn remove_user(&mut self, user_id: &str) -> Result<(), String> {
        let user = self
            .users
            .iter()
            .find(|u| u.id == user_id)
            .ok_or_else(|| format!("User {} not found in ressource {}", user_id, self.name))?;
        // Remove authorized key
        let path = self.authorized_keys_path();
        let mut authorized_keys = AuthorizedKeys::from_path(path.as_str())?;
        authorized_keys.remove_key_by_id(&user.id);
        debug!("authorized_keys: {:?}", authorized_keys);
        authorized_keys.save(path.as_str())?;

        // Kill active sessions
        kill_all_sessions(&self.name, &user.public_key.key)?;

        // Remove user from database
        self.users.retain(|u| u.id != user_id);
        Ok(())
    }

    pub fn add_all_users(&self) -> Result<(), String> {
        let path = self.authorized_keys_path();
        let mut authorized_keys = AuthorizedKeys::from_path(path.as_str())?;
        for user in &self.users {
            let authorized_key = AuthorizedKey::new(self, user)?;
            authorized_keys.add_key(authorized_key);
        }
        debug!("authorized_keys after: {:?}", authorized_keys);
        authorized_keys.save(path.as_str())?;
        Ok(())
    }
}

impl SSHRessource {
    fn authorized_keys_path(&self) -> String {
        format!("/home/{}/.ssh/authorized_keys", self.name)
    }
}
