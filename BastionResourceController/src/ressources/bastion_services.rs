use k8s_openapi::api::core::v1::Service;
use kube::{Api, Client};
use kube::api::{DeleteParams, PostParams};

pub struct BastionPublicService {
    pub bastion_id: String,
    pub ssh_port: u16,
    pub wireguard_port: u16,
}

pub struct BastionInternService {
    pub bastion_id: String,
}

impl BastionPublicService {
    pub fn new(bastion_id: String, ssh_port: u16, wireguard_port: u16) -> Self {
        Self {
            bastion_id,
            ssh_port,
            wireguard_port,
        }
    }

    pub async fn create(&self, client: Client) -> Result<(), kube::Error> {
        let services: Api<Service> = Api::namespaced(client, "bastion");
        let pp = PostParams::default();

        let bastion_service_desc = serde_json::json!({
          "apiVersion": "v1",
          "kind": "Service",
          "metadata": {
            "name": format!("bastion-{}", self.bastion_id),
            "namespace": "bastion",
            "labels": {
              "app": "bastion",
              "id": self.bastion_id.to_string()
            }
          },
          "spec": {
            "type": "LoadBalancer",
            "ports": [
              {
                "name": "ssh",
                "port": self.ssh_port,
                "protocol": "TCP",
                "targetPort": 22
              },
              {
                "name": "wireguard",
                "port": self.wireguard_port,
                "protocol": "UDP",
                "targetPort": 60244
              }
            ],
            "selector": {
              "app": "bastion",
              "id": self.bastion_id.to_string()
            }
          }
        });

        services.create(
            &pp,
            &serde_json::from_value(bastion_service_desc).map_err(|e| kube::Error::SerdeError(e))?,
        ).await?;
        Ok(())
    }

    pub async fn delete(&self, client: Client) -> Result<(), kube::Error> {
        let services: Api<Service> = Api::namespaced(client, "bastion");
        let dp = DeleteParams::default();

        services.delete(
            &format!("bastion-{}", self.bastion_id),
            &dp,
        ).await?;
        Ok(())
    }
}

impl BastionInternService {
    pub fn new(bastion_id: String) -> Self {
        Self {
            bastion_id,
        }
    }

    pub async fn create(&self, client: Client) -> Result<(), kube::Error> {
        let services: Api<Service> = Api::namespaced(client, "bastion");
        let pp = PostParams::default();

        let bastion_service_desc = serde_json::json!({
          "apiVersion": "v1",
          "kind": "Service",
          "metadata": {
            "name": format!("bastion-internal-{}", self.bastion_id),
            "namespace": "bastion",
            "labels": {
              "app": "bastion",
              "id": self.bastion_id.to_string()
            }
          },
          "spec": {
            "type": "ClusterIP",
            "ports": [
              {
                "name": "api",
                "port": 9000,
                "protocol": "TCP",
                "targetPort": 9000
              },
            ],
            "selector": {
              "app": "bastion",
              "id": self.bastion_id.to_string()
            }
          }
        });

        services.create(
            &pp,
            &serde_json::from_value(bastion_service_desc).map_err(|e| kube::Error::SerdeError(e))?,
        ).await?;
        Ok(())
    }

    pub async fn delete(&self, client: Client) -> Result<(), kube::Error> {
        let services: Api<Service> = Api::namespaced(client, "bastion");
        let dp = DeleteParams::default();

        services.delete(
            &format!("bastion-internal-{}", self.bastion_id),
            &dp,
        ).await?;
        Ok(())
    }
}