# bastion_backend

## Description

Cette API gère les données rentrées depuis le front et par les autres API.
Elle stocke en base de donnée les informations des bastions, des ressources et des utilisateurs.
Elle permet ensuite de récupérer ces informations pour les afficher sur le front, ou de les envoyer aux autres API.

## Installation

il faut mettre en place une base de donnée postgresql, lui donner un nom, ainsi qu'un utilisateur et son mot de passe.
ces données seront à renseigner dans la varible d'environnement DATABASE_URL, du format suivant :

```bash
DATABASE_URL=postgresql://user:password@host:port/database
```

il faut aussi avoir installé diesel-cli afin de manipuler cette dernière.


il faut enfin créer un fichier .env à la racine du projet, avec les variables d'environnement suivantes :

```bash
DATABASE_URL
FIRST_PORT
FIRST_NET_ID
FIRST_USER_NET_ID
AUTHENTICATION_USER
AUTHENTICATION_ADMIN
```