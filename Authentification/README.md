# Micro service de l'authentification

Lorqu'un utilisateur ou un administrateur s'est authentifié en rentrant son login/mot de passe + 2FA ou a utilisé le SSO, le service insère un JWT dans le header Authorization de la réponse : `Authorization: Bearer <JWT>`.  
  
 
Les variables d'environnement nécessaires pour faire fonctionner le service sont :  
`DATABASE_URL=postgres://bastion:PasswordOfBastion@postgresqlnfs:5432/authentication`  
`KEY_JWT="Secret du JWT"`  
`KEY_BDD="Secret sur 32 octets"`  
`NONCE="Secret sur 12 octets"`  
`REDIRECT_URL_ADMIN="https://bastionmania.intra/login/admin/extern/next"`  
`REDIRECT_URL_USER="https://bastionmania.intra/login/extern/next"`  
`URL_USER_MANAGEMENT="http://user-management:80/extern/users"`  
  
 
Avec PasswordOfBastion le mot de passe de l'utilisateut bastion de la base de données créé lors de la création de celle-ci.  
Le secret du JWT doit être une chaine aléatoire sur un minimum de 32 octets.  
La clé de chiffrement de l'AES est une chaine de caractères générée aléatoirement sur 32 octets précisément et le secret du Nonce doit être une chaine alétoire sur 12 octets.
