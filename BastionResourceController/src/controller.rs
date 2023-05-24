use futures::TryStreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    runtime::{watcher, watcher::Event},
    Api, CustomResource, Client,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::ressources::{BastionInternService, BastionPod, BastionPublicService};

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

impl BastionSpec{
    fn bastion_pod(&self) -> BastionPod {
        BastionPod::new(
            self.image.clone(),
            self.bastion_id.clone(),
            self.net_id.clone(),
        )
    }

    fn public_service(&self) -> BastionPublicService{
        BastionPublicService::new(
            self.bastion_id.clone(),
            self.ssh_port,
            self.wireguard_port
        )
    }

    fn intern_service(&self) -> BastionInternService{
        BastionInternService::new(
            self.bastion_id.clone()
        )
    }

    pub async fn apply(&self, client: Client) -> Result<(), kube::Error> {
        log::info!("Applying bastion {}", self.bastion_id);
        log::debug!("Creating pod of {}", self.bastion_id);
        self.bastion_pod().apply_pod(client.clone()).await?;
        log::debug!("Creating public_service of {}", self.bastion_id);
        self.public_service().create(client.clone()).await?;
        log::debug!("Creating intern_service of {}", self.bastion_id);
        self.intern_service().create(client.clone()).await?;

        Ok(())
    }

    pub async fn delete(&self, client: Client) -> Result<(), kube::Error> {
        log::info!("Deleting bastion {}", self.bastion_id);
        log::debug!("Deleting pod of {}", self.bastion_id);
        self.bastion_pod().delete_pod(client.clone()).await?;
        log::debug!("Deleting public_service of {}", self.bastion_id);
        self.public_service().delete(client.clone()).await?;
        log::debug!("Deleting intern_service of {}", self.bastion_id);
        self.intern_service().delete(client.clone()).await?;

        Ok(())
    }
}

pub async fn watch_bastion(client: Client) -> Result<(), watcher::Error> {
    let bastions: Api<Bastion> = Api::namespaced(client.clone(), "bastion");
    let pods: Api<Pod> = Api::namespaced(client.clone(), "bastion");
    watcher(bastions, watcher::Config::default())
        .try_for_each(|event| async {
            match event {
                Event::Applied(bn) => {
                    if let None = pods.get_opt(&format!("bastion-{}", bn.spec.bastion_id)).await.unwrap() {
                        bn.spec.apply(client.clone()).await.unwrap();
                    }else{
                        log::warn!("Bastion {} already exists", bn.spec.bastion_id);
                    }
                }
                Event::Deleted(bn) => {
                    log::info!("Bastion deleted: {}", bn.spec.bastion_id);
                    bn.spec.delete(client.clone()).await.unwrap();
                }
                Event::Restarted(bns) => {
                    log::warn!("BastionController restarted: nb bastions : {}", bns.len());
                    for bn in bns {
                        if let None = pods.get_opt(&format!("bastion-{}", bn.spec.bastion_id)).await.unwrap() {
                            bn.spec.apply(client.clone()).await.unwrap();
                        }
                    }
                }
            }
            Ok(())
        })
        .await
}
