use k8s_openapi::api::core::v1::Pod;
use tracing::debug;
use crate::BastionConfig;
use serde_json::json;

pub fn create_bastion_pod(bastion_id: i32, bastion_config: &BastionConfig) -> Result<Pod, String> {
    debug!("Creating bastion pod for bastion {}", bastion_id);
    serde_json::from_value::<Pod>(json!(
        {
  "apiVersion": "v1",
  "kind": "Pod",
  "metadata": {
    "name": format!("bastion-{}", bastion_id),
    "namespace": "bastion"
  },
  "spec": {
    "containers": [
      {
        "image": "ghcr.io/bastionmania/bastionmania/bastion:dev",
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
          }
        ],
        "resources": {
          "requests": {
            "cpu": "200m"
          }
        },
        "env": [
          {
            "name": "BASTION_PRIVATE_KEY",
            "value": &bastion_config.private_key
          },
          {
            "name": "AGENT_ENDPOINT",
            "value": &bastion_config.agent_endpoint
          },
          {
            "name": "AGENT_PUBLIC_KEY",
            "value": &bastion_config.agent_public_key
          },
          {
            "name": "NET_CIDR",
            "value": &bastion_config.cidr_protege
          },
          {
            "name": "NET_ID",
            "value": bastion_config.net_id.to_string()
          }
        ],
        "securityContext": {
          "capabilities": {
            "add": [
              "NET_ADMIN"
            ]
          }
        }
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
    "imagePullSecrets": [
      {
        "name": "repogithub"
      }
    ],
    "restartPolicy": "Always"
  }
}
    )).map_err(|e| e.to_string())
}