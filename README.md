# Bastion Mania
Bastion Mania est une solution de gestion de bastions centralisé dans un cluster kubernetes.

## Deploiement des serveurs
Pour deployer les serveur il faut suivre les scripte Ansible dans le dossier Ansible

## Deploiement du cluster
Pour deployer les services sur le cluster il faut appliquer les YAML dans le dossier Kubernetes.

## Compilation du projet
Les micros services sont écrit en rust. Ils sont testés avec rust 1.69. Un Dockerfile est dans chaque microservice pour build et avoir une image OCI minimale.

Le build est fait automatiquement avec les Actions Github.

## Utilisation du projet
### Premier lancement
Il faut aller sur bastionmania.intra/login/installation et suivre les instructions

### Ajout d'utilisateur/admin
Un admin peut ajouter des utilisateur et des admins

### Ajout de bastion
#### UI
Pour créer un Bastion il faut lui donner un nom et indiquer le CIDR du réseau protégé par ce bastion
#### Agent
Lors de la creation du bastion un token à usage unique est renvoyé, il faut l'utilliser dans la config de l'agent. Il faut aussi recupérer le certificat de l'API et le mettre sur l'agent. Une fois l'agent lancé on peut passer aux ressources.

### Ajout ressources
Un administrateur peut ajouter des ressources wireguard ou ssh, ssh donne un accès distant à une machine, un accès wireguard donne un accès à tous les port d'une machine.

### Authorisation
Un admin doit ensuite ajouter un utilisateur sur une ressource pour que l'utilisateur puisse la voire dans sa liste de bastion.

### Transmission credential user

### Activation session

### stop session

### accès ressources
#### wireguard
#### SSH
