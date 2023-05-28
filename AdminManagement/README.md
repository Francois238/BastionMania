# Micro-service pour la gestion des administrateurs
  
 Ce service s'occupe de la gestion des administrateurs de Bastion Mania.  
   
 Ce service nécessite les variables d'environnement suivantes pour fonctionner :
 `DATABASE_URL=postgres://bastion:PasswordOfBastion@postgresqlnfs:5432/gestion_admin`  
 `KEY_JWT="Secret du JWT"`  
 `AUTHENTICATION_URL=http://authentication:80/`  
   
  Avec PasswordOfBastion le mot de créé lors de l'installation de la base de données.  
  Le secret du JWT doit être identique à celui du service authentication (chaine de caractères sur au moins 32 octets générée aléatoirement).
