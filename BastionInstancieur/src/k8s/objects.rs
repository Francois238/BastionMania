use k8s_openapi::api::core::v1::{Pod, Service};
use serde_json::json;
use tracing::debug;
use crate::BastionConfig;

pub fn create_bastion_pod(bastion_id: i32, bastion_config: &BastionConfig) -> Result<Pod, String> {
    debug!("Creating bastion pod for bastion {}", bastion_id);
    serde_json::from_value::<Pod>(json!(
{
  "apiVersion": "v1",
  "kind": "Pod",
  "metadata": {
    "name": format!("bastion-{}", bastion_id),
    "namespace": "bastion",
    "labels": {
      "type": "bastion",
      "id": bastion_id.to_string()
    }
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

pub fn create_bastion_service_intern(bastion_id: i32) -> Result<Service, String> {
    debug!("Creating bastion service api for bastion {}", bastion_id);
    serde_json::from_value::<Service>(json!(
{
  "apiVersion": "v1",
  "kind": "Service",
  "metadata": {
    "name": format!("intern-bastion-{}", bastion_id),
    "namespace": "bastion"
  },
  "spec": {
    "ports": [
      {
        "port": 9000,
        "protocol": "TCP",
        "targetPort": 9000
      }
    ],
    "selector": {
      "type": "bastion",
      "id": bastion_id.to_string()
    },
    "type": "ClusterIP"
  }
}
    )).map_err(|e| format!("Failed to create bastion service: {}", e))
}

pub fn create_bastion_service_ingress(bastion_id: i32, bastion_config: &BastionConfig) -> Result<Service, String> {
    debug!("Creating bastion service ingress for bastion {}", bastion_id);
    let ext_ip = std::env::var("EXTERNAL_IP").expect("EXTERNAL_IP must be set");
    serde_json::from_value::<Service>(json!(
{
  "apiVersion": "v1",
  "kind": "Service",
  "metadata": {
    "name": format!("ingress-bastion-{}", bastion_id),
    "namespace": "bastion"
  },
  "spec": {
    "ports": [
      {
        "port": bastion_config.bastion_port,
        "protocol": "UDP",
        "targetPort": 60244
      }
    ],
    "selector": {
      "type": "bastion",
      "id": bastion_id.to_string()
    },
    "type": "LoadBalancer",
    "externalIPs": [
      ext_ip
    ]
  }
}
    )).map_err(|e| format!("Failed to create bastion service: {}", e))
}