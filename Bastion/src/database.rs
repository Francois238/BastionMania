use crate::WireguardRessource;
use crate::ssh::ressource::SSHRessource;
use serde::{Deserialize, Serialize};
use std::io;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BastionDatabase {
    ssh: Vec<SSHRessource>,
    wireguard: Vec<WireguardRessource>,
}

impl BastionDatabase {
    fn new() -> Self {
        BastionDatabase {
            ssh: Vec::new(),
            wireguard: Vec::new(),
        }
    }

    fn save(&self) -> io::Result<()> {
        let ressource_json = serde_json::to_string(&self)?;
        fs::write("/data/db.json", ressource_json)?;
        Ok(())
    }

    fn load() -> io::Result<Self> {
        let ressource_json = fs::read_to_string("/data/db.json")?;
        let ressource: BastionDatabase = serde_json::from_str(&ressource_json)?;
        Ok(ressource)
    }

    pub fn get() -> io::Result<Self> {
        if !Path::new("/data/db.json").exists() {
            return Ok(BastionDatabase::new());
        }
        BastionDatabase::load()
    }
}

impl BastionDatabase {
    pub fn add_ssh(&mut self, ressource: SSHRessource) -> io::Result<()> {
        self.ssh.push(ressource);
        self.save()
    }
    
    pub fn remove_ssh(&mut self, id: &str) -> io::Result<()> {
        self.ssh.retain(|r| r.id != id);
        self.save()
    }

    pub fn get_ssh_by_name(&self, name: &str) -> Option<&SSHRessource> {
        self.ssh.iter().find(|r| r.name == name)
    }

}
