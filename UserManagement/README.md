# Micro-service pour la gestion des utilisateurs  
## Description  
  
 Ce service s'occupe de la gestion des utilisateurs de Bastion Mania.  
   
 Ce service nécessite les variables d'environnement suivantes pour fonctionner :
 `DATABASE_URL=postgres://bastion:PasswordOfBastion@postgresqlnfs:5432/gestion_user`  
 `KEY_JWT="Secret du JWT"`  
 `AUTHENTICATION_URL=http://authentication:80/`  
   
 ## Installation
   
  Avec PasswordOfBastion le mot de créé lors de l'installation de la base de données.  
  Le secret du JWT doit être identique à celui du service authentication (chaine de caractères sur au moins 32 octets générée aléatoirement).  
Voici le squelette du secret à déployer dans votre cluster Kubernetes, nommez-le par exemple Config-User.yaml :  
```
apiVersion: v1
kind: Secret
metadata:
  name: gestionuser.conf
  namespace: bastion
type: Opaque
stringData:
  fichier.properties: |
    DATABASE_URL=postgres://bastion:PasswordOfBastion@postgresqlnfs:5432/gestion_user
    AUTHENTICATION_URL=http://authentication:80/
    KEY_JWT="Secret du JWT"
```  
  
Faites `kubectl -f Config-User.yaml`  
  
Voici le fichier de déploiement du micro-service, appelez le User-deployment.yaml :  
```
apiVersion: v1
kind: Service
metadata:
  name: user-management
  namespace: bastion
spec:
  ports:
  - port: 80 
    protocol: TCP
    targetPort: 8082
  selector:
    app: user-management
  type: ClusterIP

---


apiVersion: apps/v1
kind: Deployment
metadata:
  name: user-management
  namespace: bastion
spec:
  replicas: 1
  selector:
    matchLabels:
      app: user-management
  template:
    metadata:
      labels:
        app: user-management
    spec:
      containers:
      - name: user-management
        image: "ghcr.io/bastionmania/bastionmania/usermanagement:dev"
        imagePullPolicy: Always
        ports:
        - name: http
          protocol: TCP
          containerPort: 8082
        volumeMounts:
          - name: mnt
            mountPath: /.env
            subPath: fichier.properties
      volumes:
      - name: mnt
        secret:
          secretName: gestionuser.conf
      imagePullSecrets:
        - name: repogithub
```  
  
Faites ensuite `kubectl apply -f User-deployment.yaml`.  
Le micro-service est déployé sur votre cluster.
