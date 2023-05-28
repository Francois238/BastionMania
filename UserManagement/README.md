# Micro-service pour la gestion des utilisateurs
  
 Ce service s'occupe de la gestion des utilisateurs de Bastion Mania.  
   
 Ce service nécessite les variables d'environnement suivantes pour fonctionner :
 `DATABASE_URL=postgres://bastion:PasswordOfBastion@postgresqlnfs:5432/gestion_user`  
 `KEY_JWT="Secret du JWT"`  
 `AUTHENTICATION_URL=http://authentication:80/`  
   
  Avec PasswordOfBastion le mot de créé lors de l'installation de la base de données.  
  Le secret du JWT doit être identique à celui du service authentication (chaine de caractères sur au moins 32 octets générée aléatoirement).
