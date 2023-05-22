use crate::BastionConfig;
use kube::api::{DeleteParams, PostParams};
use kube::{Api, Client, CustomResource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize, CustomResource, JsonSchema, Default)]
#[kube(
    group = "bastionmania.fr",
    version = "v1alpha1",
    kind = "Bastion",
    plural = "bastions",
    shortname = "bs",
    namespaced
)]
pub struct BastionSpec {
    pub image: String,
    pub ssh_port: u16,
    pub wireguard_port: u16,
    pub bastion_id: String,
    pub net_id: u8,
}

impl BastionSpec {
    fn spec(&self) -> Result<Bastion, serde_json::Error> {
        serde_json::from_value(json! {
            {
                "apiVersion": "bastionmania.fr/v1alpha1",
                "kind": "Bastion",
                "metadata": {
                    "name": format!("bastion-{}", self.bastion_id),
                    "namespace": "bastion"
                },
                "spec": {
                    "image": self.image,
                    "ssh_port": self.ssh_port,
                    "wireguard_port": self.wireguard_port,
                    "bastion_id": self.bastion_id,
                    "net_id": self.net_id
                }
            }
        })
    }
    pub fn new(bastion_config: BastionConfig, image: &str) -> Self {
        Self {
            image: image.to_string(),
            ssh_port: bastion_config.ssh_port,
            wireguard_port: bastion_config.wireguard_port,
            bastion_id: bastion_config.bastion_id,
            net_id: bastion_config.net_id,
        }
    }

    pub async fn create(&self, client: Client) -> Result<(), kube::Error> {
        log::info!("Creating bastion {}", self.bastion_id);

        let bastions: Api<Bastion> = Api::namespaced(client.clone(), "bastion");

        let bastion = self.spec().map_err(kube::Error::SerdeError)?;

        let pp = PostParams::default();
        bastions.create(&pp, &bastion).await?;
        Ok(())
    }

    pub async fn delete(bastion_id: &str, client: Client) -> Result<(), kube::Error> {
        log::info!("Deleting bastion {}", bastion_id);

        let bastions: Api<Bastion> = Api::namespaced(client.clone(), "bastion");

        let dp = DeleteParams::default();
        bastions.delete(bastion_id, &dp).await?;
        Ok(())
    }
}
