use crate::ssh::ressource::SSHRessource;
use crate::{WireguardAgent, WireguardRessource};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BastionDatabase {
    ssh: Vec<SSHRessource>,
    wireguard: Vec<WireguardRessource>,
    agent: Option<WireguardAgent>,
}

impl BastionDatabase {
    fn new() -> Self {
        BastionDatabase {
            ssh: Vec::new(),
            wireguard: Vec::new(),
            agent: None,
        }
    }

    pub fn save(&self) -> io::Result<()> {
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
    pub fn exists() -> bool {
        Path::new("/data/db.json").exists()
    }
}

impl BastionDatabase {
    pub fn add_ssh(&mut self, ressource: SSHRessource) -> io::Result<()> {
        self.ssh.push(ressource);
        self.save()
    }

    pub fn remove_ssh_by_name(&mut self, name: &str) -> io::Result<()> {
        self.ssh.retain(|r| r.name != name);
        self.save()
    }

    pub fn get_ssh_by_name(&self, name: &str) -> Option<&SSHRessource> {
        self.ssh.iter().find(|r| r.name == name)
    }

    pub fn get_ssh_mut_by_name(&mut self, name: &str) -> Option<&mut SSHRessource> {
        self.ssh.iter_mut().find(|r| r.name == name)
    }

    pub fn get_ssh_ressources(&self) -> &Vec<SSHRessource> {
        &self.ssh
    }
}

impl BastionDatabase {
    pub fn add_wireguard(&mut self, ressource: WireguardRessource) -> io::Result<()> {
        self.wireguard.push(ressource);
        self.save()
    }

    pub fn wireguard_exists(&self, ressource: WireguardRessource) -> bool {
        self.wireguard.iter().any(|r| r.id == ressource.id && r.client_id == ressource.client_id)
    }

    pub fn get_wireguard_ressource(&self, id: &str, client_id: &str) -> Option<&WireguardRessource> {
        self.wireguard.iter().find(|r| r.id == id && r.client_id == client_id)
    }

    pub fn remove_wireguard(&mut self, id: &str, client_id: &str) -> io::Result<()> {
        self.wireguard.retain(|r| r.id != id && r.client_id != client_id);
        self.save()
    }

    pub fn get_wireguard_ressources(&self) -> &Vec<WireguardRessource> {
        &self.wireguard
    }
}

impl BastionDatabase {
    pub fn set_agent(&mut self, agent: WireguardAgent) -> io::Result<()> {
        self.agent = Some(agent);
        self.save()
    }

    pub fn get_agent(&self) -> Option<&WireguardAgent> {
        self.agent.as_ref()
    }

    pub fn remove_agent(&mut self) -> io::Result<()> {
        self.agent = None;
        self.save()
    }
}