use crate::tools::api_error::ApiError;
use crate::tools::db;
use crate::schema::admins;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use google_authenticator::{GoogleAuthenticator};
use uuid::Uuid;
use crate::tools::password_management::{encrypt_password, hash_password};


#[derive(Serialize, Deserialize)]
pub struct CodeOtp { //Structure recu dans le JSON authentification + structure pouvant etre insere via AdminInserable
    pub code: String,
}


//Structure gestion des admins

#[derive(Serialize, Deserialize)]
pub struct AdminAuthentication { //Structure recu dans le JSON authentification pour authentifier l'admin
    pub mail: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdminRecu { //Structure recu dans le JSON ajouter a la bdd donc converti ensuite en AdminInserable
    pub id : Uuid,
    pub name : String,
    pub last_name : String,
    pub mail: String,
    pub password: String,
    pub claim : String, //jwt
}

#[derive( AsChangeset,Insertable, Queryable)]
pub struct Admin { //Structure recupere par requete BDD
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
pub struct AdminEnvoye { //Structure a envoye dans la reponse http
    pub id: Uuid,
    pub name: String,
    pub last_name : String,
    pub mail: String,
    pub change : Option<bool>,
    pub otpactive : Option<bool>
}

 #[derive(Serialize, Deserialize)]
pub struct AdminChangeCred { //Structure recu dans le JSON ajouter a la bdd
    pub password: String,
    pub claim: String,
}


#[derive(Serialize, Deserialize)]
pub struct AdminSupprimer { //Structure recu dans le JSON pour supprimmer admin

    pub claim: String,
}



impl AdminEnvoye {

    pub fn from_admin(admin : Admin) -> AdminEnvoye{ //Fct pour renvoyer le nom et prenom dans le JSON lors de l'insertion de l'utilisateur


        AdminEnvoye { 
            id: admin.id, 
            name: admin.name, 
            last_name: admin.last_name,
            mail: admin.mail.to_string(),
            change : admin.change, 
            otpactive : admin.otpactive 
        }
    }
    
}

impl Admin {

    pub fn from_admin_recu(admin : AdminRecu) -> Result<Admin, ApiError>{ //Creation d un admin inserable

        let password  = encrypt_password(hash_password(admin.password)?)?;

        Ok(Admin {     //Renvoie la structure qui peut etre inseree en BDD
            id : admin.id,
            name : admin.name,
            last_name : admin.last_name,
            mail : admin.mail,
            password: Some(password),
            change : None,
            otp : None,
            otpactive : None
        })

    }

    pub fn find_by_mail(mail: String) -> Result<Admin, ApiError> { //Verifier que le username de l'admin qui veut se connecter existe
        let mut conn = db::connection()?;

        //Fonction pour vérifier si le username existe bien

        let admin = admins::table
            .filter(admins::mail.eq(mail))
            .first(&mut conn)?;


        Ok(admin)
        
    }

    pub fn create(admin: AdminRecu) -> Result<AdminEnvoye, ApiError> { //Fct pour créer un admin à partir du JSON envoyé a l'api
        let mut conn = db::connection()?;

        //On va saler + hasher mot de passe puis le chiffrer

        let admin = Admin::from_admin_recu(admin)?; //Creation d un admin inserable

        let admin = diesel::insert_into(admins::table)
            .values(admin)
            .get_result(& mut conn)?;

        let admin = AdminEnvoye::from_admin(admin);


        Ok(admin)
    }

    pub fn update_password(id: Uuid, cred: AdminChangeCred) -> Result<Self, ApiError> { //Mettre a jour donnees d un admin a partir de son id et JSON
        let mut conn = db::connection()?;

        let password = encrypt_password(hash_password(cred.password)?)?;  //Hash + sel puis chiffrer mot de passe

        let admin_verif : Admin = admins::table
        .filter(admins::id.eq(id))
        .first(&mut conn)?;

        if admin_verif.otpactive== None || admin_verif.otpactive== Some(false) { //Si le user utilise keycloack on ne peut pas changer le mot de passe
            return Err(ApiError::new(403,"Unauthorized".to_string())) //Si l'otp n'est pas active on ne peut pas changer le mot de passe
        }


        let admin = diesel::update(admins::table)
            .filter(admins::id.eq(id))
            .set((admins::password.eq(password), admins::change.eq(true)))  //modifie mot de passe en BDD
            .get_result(&mut conn)?;

        Ok(admin)
    }

    pub fn create_otp(id: Uuid, graine: String) -> Result<Self, ApiError> { //Ajouter en bdd la graine de l otp
        let mut conn = db::connection()?;

        let admin_verif : Admin = admins::table
        .filter(admins::id.eq(id))
        .first(&mut conn)?;

        if admin_verif.otpactive== Some(true) || admin_verif.otpactive== None { //Si le user utilise keycloack on ne peut pas activer l'otp
            return Err(ApiError::new(403,"Unauthorized".to_string())) //Si l'otp n'est pas active on ne peut pas changer le mot de passe
        }


        let admin = diesel::update(admins::table)
            .filter(admins::id.eq(id))
            .set((admins::otp.eq(graine), admins::otpactive.eq(true)))  //on ajoute la graine de l otp
            .get_result(&mut conn)?;

        Ok(admin)
    }

    pub fn verification_2fa(mail : String, code_verif: String) -> Result<AdminEnvoye, ApiError> { //verification de l'otp
        let mut conn = db::connection()?;


        let admin : Admin= admins::table
            .filter(admins::mail.eq(mail))
            .first(&mut conn)?;

        if admin.otpactive == Some(false) || admin.otp.is_none() || admin.otpactive == None{ //Si l'otp n'est pas active on renvoie une erreur ou utilise keycloack
            return Err(ApiError::new(403, "Interdit".to_string()))
        }

        let otp = admin.otp.clone().unwrap();

        let auth = GoogleAuthenticator::new();

        if auth.verify_code(&otp, &code_verif, 1, 0) == true{

            Ok(AdminEnvoye::from_admin(admin))
        }

        else{
            Err(ApiError::new(403, "Interdit".to_string()))
        }
    

   
    }


    pub fn delete(id: Uuid) -> Result<usize, ApiError> { //Supprimer un admin de la BDD
        let mut conn = db::connection()?;

        let res = diesel::delete(
            admins::table
                    .filter(admins::id.eq(id))
            )
            .execute(&mut conn)?;

        Ok(res)
    }

}



pub fn premiere_utilisation_bastion(admin : AdminRecu) -> Result<AdminEnvoye, ApiError>  { //Fct pour creer l'admin par défaut 

    let mut conn = db::connection().expect("Erreur connexion BDD");

    let admins = admins::table
            .load::<Admin>(&mut conn).expect("Erreur connexion BDD");

    if admins.len() == 0 { //Si aucun utilisateur, on creer l'utilisateur par défaut

        let admin = Admin::create(admin).expect("Erreur connexion BDD"); //Insertion de cet utilisateur dans la BDD
    
        Ok(admin)

    }
    else{
        Err(ApiError { status_code: 404, message: " ".to_string() })
    }

}
