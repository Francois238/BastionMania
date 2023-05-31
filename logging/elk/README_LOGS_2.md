# README 2 : LOGS 2

Ceci est le README de ELK la deuxieme solution testee par notre equipe et presentee a la slide 77

# Etat des lieux

Nous ne devons pas chercher comment collecter les logs pour un docker host avec plusieurs containers running.  
Nous devons chercher comment le faire pour un cluster kubernetes avec plusieurs pods.  
Les logs API seront sur le STDIN. Mais les logs SSH ne seront pas sur le STDIN. 

# I Logstash pour recuperer les logs de notre cluster kubernetes

Emplacement du fichier de config de notre cluster : /home/.kube

## 1 Installer Logstash selon la documentation officielle (https://www.elastic.co/guide/en/logstash/current/installing-logstash.html)[https://www.elastic.co/guide/en/logstash/current/installing-logstash.html]

2-Configurez Logstash pour collecter les logs envoyes par le cluster :  
        Telechargez et installez Logstash a partir du site web officiel d'Elasticsearch.
        Creez un fichier de configuration Logstash qui definit une entree Beats pour recevoir les logs envoyes par Filebeat et une sortie Elasticsearch pour stocker les logs traités. Vous pouvez ajouter des filtres pour extraire les informations pertinentes des logs.

## 2 Installer & configurer Filebeat afin que le cluster puisse envoyer ses logs à Logstash

 Sur chaque node du cluster:

### Installer Filebeat pour collecter les logs  
Via le site officiel de elastic (=ELK) (https://www.elastic.co/guide/en/beats/filebeat/current/filebeat-installation-configuration.html)[https://www.elastic.co/guide/en/beats/filebeat/current/filebeat-installation-configuration.html]  
Nous avons donc telecharge & installe la derniere version :   
curl -L -O https://artifacts.elastic.co/downloads/beats/filebeat/filebeat-8.7.1-linux-x86_64.tar.gz 

tar xzvf filebeat-8.7.1-linux-x86_64.tar.gz 

### Configurer Filebeat pour qu'il envoie les logs a Logstash en utilisant le protocole Beats.  
Specifier l'adresse IP et le port de Logstash dans le fichier de configuration de Filebeat.  
Pour cela modifier le fichier /filebeat-8.7.1-linux-x86_64/filebeat.yml au niveau de Logstach Output.    

Specifier le chemin vers le fichier de configuration de notre cluster Kubernetes situe dans /home/.kube/config.  
Pour cela modfier le fichier /filebeat-8.7.1-linux-x86_64/filebeat.yml au niveau de Logstach Input.   

### Demarrer Filebeat  
sudo ./filebeat -c filebeat.yml

## 3 Configurer & demarrer Logstash pour recevoir les logs de Filebeat  
Creer un fichier de configuration Logstash /etc/logstash/conf.d/filebeat.conf. 
Cela enverra les logs traites a Elasticsearch, en utilisant l'index my-index suivi de la date.

### Demarrer Logstash :  
Demarrer Logstash en utilisant la commande suivante  
sudo systemctl start logstash  

# II Installer & configurer Elasticsearch pour stocker les logs traites  

## 1 Installer Elasticsearch

Via le site officiel de elastic (=ELK) (https://www.elastic.co/guide/en/elasticsearch/reference/current/install-elasticsearch.html)[https://www.elastic.co/guide/en/elasticsearch/reference/current/install-elasticsearch.html]

## 2 Configurer Elasticsearch pour qu'il cree des index dedies pour stocker les logs traites  

Pour cela specifier des parametres d'indexation tels que le nombre de shards et de replicas, ainsi que les mappings de champ.  
Ouvrir le fichier de configuration Elasticsearch /etc/elasticsearch/elasticsearch.yml au niveau de Index.
*index.name: C'est le nom de l'index. Dans l'exemple donné, le nom de l'index est défini comme "logs-" suivi du format de date "%{+yyyy.MM.dd}". Cela signifie qu'un nouvel index sera créé chaque jour, avec une date au format "yyyy.MM.dd". Par exemple, un index créé le 17 mai 2023 aura le nom "logs-2023.05.17".
*index.number_of_shards: C'est le nombre de shards (fragments) dans lesquels l'index sera divisé. Un shard est une unité de traitement parallèle dans Elasticsearch. Plus le nombre de shards est élevé, plus vous pouvez répartir la charge de recherche et d'indexation. Dans l'exemple donné, l'index est configuré avec 3 shards.
*index.number_of_replicas: C'est le nombre de replicas (répliques) de chaque shard de l'index. Une réplique est une copie d'un shard qui permet d'améliorer la disponibilité et la résilience du système en cas de défaillance. Dans l'exemple donné, chaque shard de l'index est répliqué une fois, ce qui signifie qu'il y aura une réplique pour chaque shard.

# III Configurez Kibana pour afficher et analyser les logs :

## 1 Installer Kibana

Via le site officiel de elastic (=ELK) (https://www.elastic.co/guide/en/kibana/current/install.html)[https://www.elastic.co/guide/en/kibana/current/install.html]

## 2 Connectez Kibana à Elasticsearch : Une fois que Kibana est installé, vous devez le connecter à votre cluster Elasticsearch pour pouvoir visualiser et analyser les logs. Pour ce faire, vous aurez besoin de l'URL d'Elasticsearch ainsi que des informations d'authentification appropriées si votre cluster Elasticsearch est sécurisé.

Dans le fichier de configuration de Kibana kibana.conf
Au niveau de elasticsearch.hosts.
Spécifiez l'URL d'Elasticsearch http://localhost:9200 comme configure dans filebeat.conf.
Si nécessaire, ajoutez également les informations d'authentification, telles que les noms d'utilisateur et les mots de passe, dans les paramètres de connexion.
Utiliser l'interface utilisateur de Kibana
Une fois que Kibana est connecté à Elasticsearch, nous pourrons creer des visualisations et des tableaux de bord personnalises pour afficher et analyser les logs.

Nous accedons a l'interface utilisateur de Kibana via l'URL de Kibana (par défaut http://localhost:5601).
Ce qui permet d'explorer et analyser les logs.