use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client, api::{PostParams, DeleteParams}};

pub struct BastionPod {
    image: String,
    bastion_id: String,
    net_id: u8,
}

impl BastionPod {
    pub fn new(image: String, bastion_id: String, net_id: u8) -> Self {
        Self {
            image,
            bastion_id,
            net_id,
        }
    }

    pub async fn apply_pod(&self, client: Client) -> Result<(), kube::Error> {
        let pods: Api<Pod> = Api::namespaced(client, "bastion");
        let pp = PostParams::default();

        let bastion_pod_desc = serde_json::json!({
          "apiVersion": "v1",
          "kind": "Pod",
          "metadata": {
            "name": format!("bastion-{}", self.bastion_id),
            "namespace": "bastion",
            "labels": {
              "app": "bastion",
              "id": self.bastion_id.to_string()
            }
          },
          "spec": {
            "containers": [
              {
                "image": self.image.to_string(),
                "imagePullPolicy": "Always",
                "name": "bastion",
                "ports": [
                  {
                    "containerPort": 9000,
                    "protocol": "TCP"
                  },
                  {
                    "containerPort": 60244,
                    "protocol": "UDP"
                  },
                  {
                    "containerPort": 22,
                    "protocol": "TCP"
                  }
                ],
                "env": [
                  {
                    "name": "NET_ID",
                    "value": self.net_id.to_string()
                  }
                ],
                "securityContext": {
                  "capabilities": {
                    "add": [
                      "NET_ADMIN"
                    ]
                  }
                },
                  "volumeMounts": [
                    {
                      "mountPath": "/data",
                      "name": "data"
                    }
                  ]
              }
            ],
            "initContainers": [
              {
                "name": "init-sys",
                "securityContext": {
                  "privileged": true
                },
                "image": "busybox",
                "command": [
                  "sh",
                  "-c",
                  "echo 1 > /proc/sys/net/ipv4/ip_forward"
                ]
              }
            ],
            "volumes": [
              {
                "name": "data",
                "hostPath": {
                  "path": format!("/data/{}", self.bastion_id),
                  "type": "DirectoryOrCreate"
                }
              }
            ],
            "imagePullSecrets": [
              {
                "name": "repogithub"
              }
            ],
            "restartPolicy": "Always"
          }
        });

        let pod: Pod = serde_json::from_value(bastion_pod_desc).unwrap();

        let _ = pods.create(&pp, &pod).await?;

        Ok(())
    }

    pub async fn delete_pod(&self, client: Client) -> Result<(), kube::Error> {
        let pods: Api<Pod> = Api::namespaced(client, "bastion");
        let dp = DeleteParams::default();

        let _p = pods.delete(&format!("bastion-{}", self.bastion_id), &dp).await?;

        Ok(())
    }
}
