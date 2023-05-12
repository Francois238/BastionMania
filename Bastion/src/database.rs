use crate::ssh::ressource::SSHRessource;
use serde::{Deserialize, Serialize};
use std::io;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BastionDatabase {
    resources: Vec<Resource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub protocol: Protocol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    SSH(SSHRessource),
    Wireguard,
}

impl BastionDatabase {
    fn new() -> Self {
        BastionDatabase {
            resources: Vec::new(),
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
    pub fn add_resource(&mut self, resource: Resource) -> io::Result<()> {
        self.resources.push(resource);
        self.save()
    }

    pub fn remove_resource(&mut self, resource_id: &str) -> io::Result<()> {
        self.resources.retain(|r| r.id != resource_id);
        self.save()
    }

    pub fn get_resource(&self, resource_id: &str) -> Option<&Resource> {
        self.resources.iter().find(|r| r.id == resource_id)
    }

    pub fn get_resources(&self) -> &Vec<Resource> {
        &self.resources
    }

    pub fn get_resources_ssh(&self) -> Vec<&Resource>
     {
        self.resources
            .iter()
            .filter(|r| {
                if let Protocol::SSH(_) = r.protocol {
                    true
                } else {
                    false
                }
            })
            .collect()
    }

    pub fn get_resources_wireguard(&self) -> Vec<&Resource> {
        self.resources
            .iter()
            .filter(|r| {
                if let Protocol::Wireguard = r.protocol {
                    true
                } else {
                    false
                }
            })
            .collect()
    }
}
