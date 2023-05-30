# Micro service de l'authentification

## Description 

Lorsqu'un utilisateur ou un administrateur s'est authentifié en rentrant son login/mot de passe + 2FA ou a utilisé le SSO, le service insère un JWT dans le header Authorization de la réponse : `Authorization: Bearer <JWT>`.  
  
 
Les variables d'environnement nécessaires pour faire fonctionner le service sont :  
`DATABASE_URL=postgres://bastion:PasswordOfBastion@postgresqlnfs:5432/authentication`  
`KEY_JWT="Secret du JWT"`  
`KEY_BDD="Secret sur 32 octets"`  
`NONCE="Secret sur 12 octets"`  
`REDIRECT_URL_ADMIN="https://bastionmania.intra/login/admin/extern/next"`  
`REDIRECT_URL_USER="https://bastionmania.intra/login/extern/next"`  
`URL_USER_MANAGEMENT="http://user-management:80/extern/users"`  
  
## Installation  
 
Avec PasswordOfBastion le mot de passe de l'utilisateut bastion de la base de données créé lors de la création de celle-ci.  
Le secret du JWT doit être une chaine aléatoire sur un minimum de 32 octets.  
La clé de chiffrement de l'AES est une chaine de caractères générée aléatoirement sur 32 octets précisément et le secret du Nonce doit être une chaine alétoire sur 12 octets.  
  
Voici le squelette du secret à déployer dans votre cluster Kubernetes, nommez-le par exemple Config-Authentication.yaml :  
```  
apiVersion: v1
kind: Secret
metadata:
  name: authentication.conf
  namespace: bastion
type: Opaque
stringData:
  fichier.properties: |
    DATABASE_URL=postgres://bastion:PasswordOfBastion@postgresqlnfs:5432/authentication
    KEY_JWT="Secret du JWT"
    KEY_BDD="Secret sur 32 octets"
    NONCE="Secret sur 12 octets"
    REDIRECT_URL_ADMIN="https://bastionmania.intra/login/admin/extern/next"
    REDIRECT_URL_USER="https://bastionmania.intra/login/extern/next"
    URL_USER_MANAGEMENT="http://user-management:80/extern/users"
```  
  
Faite `kubectl -f Config-Authentication.yaml`  
  
Voici le fichier de déploiement du micro-service, appelez le Authentication.yaml :  
  
```
apiVersion: v1
kind: Service
metadata:
  name: authentication
  namespace: bastion
spec:
  ports:
  - port: 80 
    protocol: TCP
    targetPort: 8080
  selector:
    app: authentication
  type: ClusterIP

---


apiVersion: apps/v1
kind: Deployment
metadata:
  name: authentication
  namespace: bastion
spec:
  replicas: 1
  selector:
    matchLabels:
      app: authentication
  template:
    metadata:
      labels:
        app: authentication
    spec:
      containers:
      - name: authentication
        image: "ghcr.io/bastionmania/bastionmania/authentification:dev"
        imagePullPolicy: Always
        ports:
        - name: http
          protocol: TCP
          containerPort: 8080
        volumeMounts:
          - name: mnt
            mountPath: /.env
            subPath: fichier.properties
      volumes:
      - name: mnt
        secret:
          secretName: authentication.conf
      imagePullSecrets:
        - name: repogithub
```  
  
Faite ensuite `kubectl apply -f Authentication.yaml`.  
Le micro-service est déployé sur votre cluster.
