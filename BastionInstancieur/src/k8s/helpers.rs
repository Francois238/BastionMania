use k8s_openapi::api::core::v1::{Pod, Service};
use kube::{Api, Client};
use kube::api::{DeleteParams, PostParams};
use tracing::info;

use crate::BastionConfig;
use crate::k8s::objects::{create_bastion_pod, create_bastion_service_ingress, create_bastion_service_intern};

pub async fn create_bastion(bastion_id: i32, bastion_config: &BastionConfig) -> Result<(), String> {
    info!("Creating bastion {}", bastion_id);
    // Create k8s client
    let client = Client::try_default().await.map_err(|e| e.to_string())?;
    // get k8s api
    let pods: Api<Pod> = Api::namespaced(client.clone(), "bastion");
    let services: Api<Service> = Api::namespaced(client.clone(), "bastion");
    let post_param = PostParams::default();

    // Create Pod bastion
    let bastion_pod = create_bastion_pod(bastion_id, bastion_config)?;
    let service_intern = create_bastion_service_intern(bastion_id)?;
    let service_ingress = create_bastion_service_ingress(bastion_id, bastion_config)?;

    post_pod(&pods, post_param.clone(), bastion_pod).await?;
    post_service(&services, post_param.clone(), service_intern).await?;
    post_service(&services, post_param.clone(), service_ingress).await?;
    Ok(())
}

pub async fn delete_bastion(bastion_id: i32) -> Result<(), String> {
    info!("Deleting bastion {}", bastion_id);
    // Create k8s client
    let client = Client::try_default().await.map_err(|e| e.to_string())?;
    // get k8s api
    let pods: Api<Pod> = Api::namespaced(client.clone(), "bastion");
    let services: Api<Service> = Api::namespaced(client.clone(), "bastion");
    let delete_param = DeleteParams::default();

    pods.delete(&format!("bastion-{}", bastion_id), &delete_param).await.map_err(|e| e.to_string())?;
    services.delete(&format!("intern-bastion-{}", bastion_id), &delete_param).await.map_err(|e| e.to_string())?;
    services.delete(&format!("ingress-bastion-{}", bastion_id), &delete_param).await.map_err(|e| e.to_string())?;

    Ok(())
}

async fn post_pod(pods: &Api<Pod>, post_param: PostParams, bastion_pod: Pod) -> Result<(), String> {
    match pods.create(&post_param, &bastion_pod).await {
        Ok(o) => {
            let name = o.metadata.name.unwrap();
            info!("Created {}", name);
        }
        Err(kube::Error::Api(ae)) => return Err(ae.to_string()),  // if you skipped delete, for instance
        Err(e) => return Err(e.to_string()),                        // any other case is probably bad
    }
    Ok(())
}

async fn post_service(services: &Api<Service>, post_param: PostParams, bastion_service: Service) -> Result<(), String> {
    match services.create(&post_param, &bastion_service).await {
        Ok(o) => {
            let name = o.metadata.name.unwrap();
            info!("Created {}", name);
        }
        Err(kube::Error::Api(ae)) => return Err(ae.to_string()),  // if you skipped delete, for instance
        Err(e) => return Err(e.to_string()),                        // any other case is probably bad
    }
    Ok(())
}