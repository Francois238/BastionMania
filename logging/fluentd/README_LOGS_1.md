# README 1 : LOGS 1

Ceci est le README de la premiere solution testee par notre equipe et presentee a la slide 76

# Etat des lieux

Nous ne devons pas chercher comment collecter les logs pour un docker host avec plusieurs containers running. Nous devons chercher comment le faire pour un cluster kubernetes avec plusieurs pods.  
Les logs API seront sur le STDIN. Mais les logs SSH ne seront pas sur le STDIN.

# I Fluentd pour recuperer les logs d'un cluster kubernetes genere avec kind en local

Les logs API seront sur le STDIN.  
Mais les logs SSH ne seront pas sur le STDIN.  
Pour que fluentd recupere les logs de plusieurs noeuds kubernetes, fluentd va run dans deamonset.  
La premiere facon de coder pour un clustere local ne s'appliquant pas avec une version non locale du cluster, il faut poursuivre les recheches pour trouver la facon de faire avec un cluster non local.  

## 1 Installer Fluentd selon la documentation officielle (https://docs.fluentd.org/installation)[https://docs.fluentd.org/installation]

Pour que fluentd recupere les logs de plusieurs noeuds kubernetes, fluentd va run dans un deamonset.

Cette premiere facon de coder pour un clustere local ne s'appliquant pas a une version non locale du cluster, il faut poursuivre les recheches pour trouver une facon de faire avec un cluster non local 

## 2 Telecharger le dossier deamonset

Le deamonset permet : to mount all the config files and location to where the logs are stored on the kubernetes node.  
Il existe le repo github suivant (https://github.com/fluent/fluentd-kubernetes-daemonset/tree/master/docker-image)[https://github.com/fluent/fluentd-kubernetes-daemonset/tree/master/docker-image] qui permet de selon la version installe de recuperer un dossier avec la configuration des deamons requise.  

## 3 Configmap

Script qui permet de recuperer les logs de chaque node (configuration pour un cluster kind) : /monitoring/logging/fluentd/kubernetes/fluentd-configmap.yaml
>>>DELETE CLUSTER : 
    kind delete cluster --name <nom du cluster>
>>>VOIR LES RUNNING CLUSTERS : 
    kind get clusters

kindest/node:v1.26.3 est la derniere version d'image kind sur le repo git officiel : https://github.com/kubernetes-sigs/kind/releases
Ce que j ai tappe depuis le debut pour que ca marche : 

### Installer kind
    curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.11.1/kind-linux-amd64
    chmod +x ./kind
 
    sudo mv ./kind /usr/local/bin/kind

    kind version

kind create cluster --name fluentd --image kindest/node:v1.26.3   
 
kubectl get nodes                                              
 
git clone https://github.com/marcel-dempers/docker-development-youtube-series.git
 
cd monitoring/logging/fluentd/kubernetes/  
 
docker build . -t aimvector/fluentd-demo
 
kubectl create ns fluentd
 
kubectl apply -f fluentd-configmap.yaml 

### Deploiement du configmap
Script qui permet de recuperer les logs de chaque node (configuration pour un cluster kind) : /monitoring/logging/fluentd/kubernetes/fluentd.yaml

 
kubectl apply -f fluentd-configmap.yaml 

### Deamonset Fluentd
kubectl apply -f fluentd-rbac.yaml 

kubectl apply -f fluentd.yaml

kubectl -n fluentd get pods

### Exemple sur une appli qui ecrit ses logs dans STDOUT 
kubectl apply -f counter.yaml  
 
kubectl get pods  
 
kubectl logs counter 

### To have a bash terminal inside of the fluentd pod
kubectl -n fluentd exec -it fluentd-g452p bash
root@fluentd-g452p:/home/fluent# cd /var/log
root@fluentd-g452p:/var/log# ls
Pour voir notre counter pod : 
root@fluentd-g452p:/var/log# ls -al containers

# II ElasticSearch & Kibana

kubectl create ns elastic-kibana

## 1 Deploiement de ElasticSearch

kubectl -n elastic-kibana apply -f .\monitoring\logging\fluentd\kubernetes\elastic\elastic-demo.yaml
kubectl -n elastic-kibana get pods

## 2 Deploiement de Kibana

kubectl -n elastic-kibana apply -f .\monitoring\logging\fluentd\kubernetes\elastic\kibana-demo.yaml
kubectl -n elastic-kibana get pods

### Kibana
kubectl -n elastic-kibana port-forward svc/kibana 5601