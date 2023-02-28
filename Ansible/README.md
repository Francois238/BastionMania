# Installation du cluster kubernetes
## Lancer les playbooks
## Initialisation du cluster
Initialisation avec kubeadm
```bash
sudo kubeadm init --control-plane-endpoint "10.10.40.4:6443" --upload-certs --pod-network-cidr="10.123.0.0/16" --config kubeadm-config.yaml
```