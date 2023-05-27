# Installation de la BDD  

## Installation du serveur NFS  
Pour installer un serveur NFS, lancez sur le serveur les commandes suivantes :  
`sudo apt update` puis `sudo apt install nfs-kernel-server`  
Créer un répertoire sur le serveur : `sudo mkdir -p /data/volume/` puis `sudo chown nobody:nogroup /data/volume/`  
Il faut maintenant rendre possible le partage de ce dossier :  
Entrez `sudo nano /etc/exports` ou avec un autre éditeur puis ajouter à la fin de ce fichier :  
`10.10.40.0/24(rw,sync,no_subtree_check,no_root_squash)` (10.10.40.0/24 correspond à votre sous-réseau de worker, à adapter si différent) 
Puis redémarrer le serveur NFS : `sudo systemctl restart nfs-kernel-server`  
  
Sur les workers du cluster, reproduisez les étapes suivantes :  
`sudo apt update` puis `sudo apt install nfs-common`  
Créer un répertoire : `sudo mkdir /data/volume/` puis `chmod 777 -R /data`  
Faite `sudo mount ip_server_nfs:/data/volume /data/volume`  



## Installation de Helm :  
Sur une VM master entrez :  
`curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3`  
`chmod 700 get_helm.sh`  
`./get_helm.sh`  
## Deploiement de postgres dans le cluster  :  
Creez dans chaque VM le dossier /data/volume  
Entrez `kubectl apply -f Postgres-pvc.yaml`  
Dans le fichier Postgres-values.yaml mettez un mot de passe pour l'administrateur de la base de données
`helm install postgresqlnfs -n bastion -f Postgres-values.yaml bitnami/postgresql`  

Dans le fichier CreateDatabase-Job.yaml, mettez un mot de passe pour l'utilisateur bastion puis mettez votre mot de passe que vous avez créer pour Postgres-values.yaml à la plage du ChangeMe au niveau des variables d'environnement.  
Puis `kubectl apply -f CreateDatabase-Job.yaml`  
  
# Deploiement de l'api gateway  
Consultez le site de [MetalLB](https://metallb.universe.tf/installation/) pour déployer MetalLb dans votre
cluster.  
Si vous souhaitez utiliser un service extérieur pour l'authentification, modifiez le fichier kong-deployment.yaml afin d'entrez vos identifiant, sinon suppprimez la section authentication-keycloak.  
Generer également un certificat TLS puis copier le ainsi que votre clé à la fin du fichier.  
Voici la commande pour créer un certificat auto-signé avec génération d'une clé :  
`openssl req -subj '/CN=bastionmania.intra' -new -newkey ec:<(openssl ecparam -name prime256v1) -sha256   -days 365 -nodes -x509 -keyout server.key -out server.crt   -addext "subjectAltName = DNS:bastionmania.intra"   -addext "keyUsage = digitalSignature"   -addext "extendedKeyUsage = serverAuth" 2> /dev/null`

Pour déployer Kong, entrez `kubectl apply -f kong-deployment.yaml`.  
  
Modifier le fichier Metallb-configuration.yaml afin de mettre votre range d'adresse IP à l'endroit indiqué dans le fichier.  
Ensuite entrez `kubectl apply -f Metallb-configuration.yaml` afin que Kong obtienne une adresse IP.  
Entrez `curl -k -F "config=@v2-kong-ingress.yml" https://10.10.40.25:8444/config`  
