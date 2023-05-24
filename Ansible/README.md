# Installation du cluster kubernetes
## Lancer les playbooks
```bash
# Si utilisation de git sur windows, il faut passer les fichiers de config en LF
ansible-playbook -i inventory.yaml install_common.yaml
ansible-playbook -i inventory.yaml install_master.yaml
ansible-playbook -i inventory.yaml install_worker.yaml
```
## Initialisation du cluster
Initialisation avec kubeadm
```bash
sudo kubeadm init --control-plane-endpoint "10.10.40.4:6443" --upload-certs --pod-network-cidr="10.123.0.0/16" --config kubeadm-config.yaml
```
## Faire rejoindre les worker
### Obtention de lla commande join
Executer sur une des master
```shell
sudo kubeadm token create --print-join-command
```
Résullltat de la forme :
```shell
kubeadm join host:port --token xxx --discovery-token-ca-cert-hash sha256:xxx
```