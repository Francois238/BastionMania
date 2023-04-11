
use crate::tools::api_error::ApiError;
use crate::tools::db;
use crate::schema::users;

use diesel::prelude::*;
use google_authenticator::GoogleAuthenticator;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tools::password_management::{encrypt_password, hash_password};


#[derive(Serialize, Deserialize)]
pub struct CodeOtp { //Structure recu dans le JSON qui contient le code otp
    pub code: String,
}

//Structure gestion des users

#[derive(Serialize, Deserialize)]
pub struct UserAuthentication { //Structure recu dans le JSON authentification pour authentifier l utilisateur
    pub mail: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserRecu { //Structure recu dans le JSON ajouter a la bdd.
    pub id : Uuid,
    pub name : String,
    pub last_name : String,
    pub mail: String,
    pub password: String,
    pub claim : String, //jwt de l admin qui ajoute l utilisateur
}

#[derive( AsChangeset,Insertable, Queryable)]
pub struct User { //Structure recupere par requete BDD
    pub id : Uuid,
    pub name : String,
    pub last_name : String,
    pub mail : String,
    pub password: Option<Vec<u8>>,
    pub change : Option<bool>,
    pub otp : Option<String>,
    pub otpactive : Option<bool>
}

#[derive(Serialize, Deserialize)]
pub struct UserEnvoye { //Structure a envoye dans la reponse http de l'admin
    pub id: Uuid,
    pub name: String,
    pub last_name : String,
    pub mail: String,
    pub change : Option<bool>,
    pub otpactive : Option<bool>
}



  #[derive(Serialize, Deserialize)]
pub struct UserChangeCred { //Structure recu dans le JSON ajouter a la bdd
    pub password: String,
    pub claim: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSupprimer { //Structure recu dans le JSON pour supprimmer user

    pub claim: String,
}



impl UserEnvoye {

    pub fn from_user(user : User) -> UserEnvoye{ //Fct pour renvoyer le nom et prenom dans le JSON lors de l'insertion de l'utilisateur

        UserEnvoye { 
            id: user.id, 
            name: user.name, 
            last_name: user.last_name, 
            mail: user.mail.to_string(), 
            change : user.change, 
            otpactive : user.otpactive 
        }
    }
    
}


impl User {

    pub fn from_user_received(user : UserRecu) -> Result<User, ApiError> { //Creation d un user inserable

        let password = encrypt_password(hash_password(user.password)?)?; //On chiffre le mot de passe

        Ok(User {     //Renvoie la structure qui peut etre inseree en BDD
            id : user.id,
            name : user.name,
            last_name : user.last_name,
            mail : user.mail,
            password : Some(password),
            change : None,  //on creer le user donc mot de passe par defaut
            otp : None,
            otpactive : None
        })
    }


    pub fn find_by_mail(mail: String) -> Result<User, ApiError> { //Verifier que le username de l'user qui veut se connecter existe
        let mut conn = db::connection()?;

        //Fonction pour vérifier si le username existe bien

        let user = users::table
            .filter(users::mail.eq(mail))
            .first(&mut conn)?;

        Ok(user)
        
    }

    pub fn create(user: UserRecu) -> Result<UserEnvoye, ApiError> { //Fct pour créer un user à partir du JSON envoyé a l'api
        let mut conn = db::connection()?;

        //On va saler + hasher mot de passe puis le chiffrer

        let user = User::from_user_received(user)?; //Creation d un admin inserable

        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(& mut conn)?;

        let user = UserEnvoye::from_user(user);


    Ok(user)
    }

    pub fn update_password(id: Uuid, cred: UserChangeCred) -> Result<Self, ApiError> { //Mettre a jour donnees d un user a partir de son id et JSON
        let mut conn = db::connection()?;

        let password = encrypt_password(hash_password(cred.password)?)?;  //Hash + sel puis chiffrer mot de passe

        let user_verif : User = users::table
        .filter(users::id.eq(id))
        .first(&mut conn)?;

        if user_verif.otpactive== None || user_verif.otpactive== Some(false) {
            return Err(ApiError::new(403,"Unauthorized".to_string())) //Si l'otp n'est pas active on ne peut pas changer le mot de passe
        }


        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set((users::password.eq(password), users::change.eq(true)))//modifie mot de passe en BDD 
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn create_otp(id: Uuid, graine: String) -> Result<Self, ApiError> { //Ajouter en bdd la graine de l otp
        let mut conn = db::connection()?;

        let user_verif : User = users::table
        .filter(users::id.eq(id))
        .first(&mut conn)?;

        if user_verif.otpactive== Some(true) || user_verif.otpactive== None{
            return Err(ApiError::new(403,"Unauthorized".to_string())) //Si l'otp est deja active on ne peut pas la creer
        }


        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set((users::otp.eq(graine), users::otpactive.eq(true)))  //on ajoute la graine de l otp
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn verification_2fa(mail : String, code_verif: String) -> Result<UserEnvoye, ApiError> { //verification de l'otp
        let mut conn = db::connection()?;


        let user : User= users::table
            .filter(users::mail.eq(mail))
            .first(&mut conn)?;

        if user.otpactive == Some(false) || user.otp.is_none() || user.otpactive == None { //Si l'otp n'est pas active on renvoie une erreur
            return Err(ApiError::new(403, "Interdit".to_string()))
        }

        let otp = user.otp.clone().unwrap(); //unwrap car on sait que c'est un Some

        let auth = GoogleAuthenticator::new();

        if auth.verify_code(&otp, &code_verif, 1, 0) == true{

            Ok(UserEnvoye::from_user(user))
        }

        else{
            Err(ApiError::new(403, "Interdit".to_string()))
        }
    

   
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> { //Supprimer un user de la BDD
        let mut conn = db::connection()?;

        let res = diesel::delete(
            users::table
                    .filter(users::id.eq(id))
            )
            .execute(&mut conn)?;

        Ok(res)
    }

}

