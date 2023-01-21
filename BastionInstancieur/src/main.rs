use std::thread;
use k8s_openapi::api::core::v1::Pod;
use serde_json::json;
use tracing::*;

use kube::{
    api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt},
    runtime::wait::{await_condition, conditions::is_pod_running},
    Client,
};
use BastionInstancieur::{BastionConfig, create_bastion_pod};

#[actix_web::main]
async fn main() -> Result<(),String> {
    tracing_subscriber::fmt::init();
    let client = Client::try_default().await.map_err(|e| e.to_string())?;

    // Manage pods
    let pods: Api<Pod> = Api::namespaced(client, "bastion");

    // Create Pod blog
    info!("Creating Pod instance blog");
    let p: Pod = create_bastion_pod(56, &BastionConfig{
        private_key: "KBuirIxRe0d8wVw/juiDE5kkCyWtDRVeQtCE+QLxn0U=".to_string(),
        cidr_protege: "10.10.50.0/24".to_string(),
        agent_public_key: "80wp0f9G/CGkYbtMN4ZmFIknOX9mO57BVo6bK5w02Ek=".to_string(),
        agent_endpoint: "10.10.40.10:60469".to_string(),
        net_id: 5,
    })?;

    let pp = PostParams::default();
    match pods.create(&pp, &p).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(p.name_any(), name);
            info!("Created {}", name);
        }
        Err(kube::Error::Api(ae)) =>  return Err(ae.to_string()),  // if you skipped delete, for instance
        Err(e) => return Err(e.to_string()),                        // any other case is probably bad
    }

    // Watch it phase for a few seconds
    thread::sleep(std::time::Duration::from_secs(5));

    // Verify we can get it
    info!("Get Pod blog");
    let p1cpy = pods.get("bastion-56").await.map_err(|e| e.to_string())?;
    if let Some(spec) = &p1cpy.spec {
        info!("Got blog pod with containers: {:?}", spec.containers);
        assert_eq!(spec.containers[0].name, "bastion-56");
    }



    Ok(())
}