use std::env;

use crate::api_error::ApiError;
use crate::db;
use crate::schema::admins;
use actix_session::Session;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};
use time::{Duration, OffsetDateTime};
use argon2::Config;
use rand::Rng;
use jsonwebtoken::{decode,  DecodingKey, Validation, Algorithm};
use google_authenticator::{GoogleAuthenticator};


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
    pub name : String,
    pub last_name : String,
    pub mail: String,
    pub password: String,
    pub claim : String, //jwt
}

#[derive(AsChangeset,Insertable)]
#[diesel(table_name = admins)]
pub struct AdminInserable { //Structure inseree en BDD pour ajouter un admin
    pub name : String,
    pub last_name : String,
    pub mail : String,
    pub password: Vec<u8>,
    pub change : bool,
    pub otpactive : bool
}

#[derive( Queryable)]
pub struct Admin { //Structure recupere par requete BDD
    pub id: i32,
    pub name : String,
    pub last_name : String,
    pub mail : String,
    pub password: Vec<u8>,
    pub change : bool,
    pub otp : Option<String>,
    pub otpactive : bool
}

#[derive(Serialize, Deserialize)]
pub struct AdminEnvoye { //Structure a envoye dans la reponse http
    pub id: i32,
    pub name: String,
    pub last_name : String,
    pub mail: String,
    pub change : bool,
    pub otpactive : bool
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Claims {  //Struture composant le JWT
    pub id: i32,
    pub name:String,
    pub last_name:String,
    pub mail : String,
    pub admin: bool,
    pub change_password : bool,
    pub mfa_active : bool,
    pub complete_authentication : bool,
    #[serde(with = "jwt_numeric_date")]
    pub iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    pub exp: OffsetDateTime,
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




 impl Claims {

    pub fn from_admin(admin : &AdminEnvoye, verif : bool) -> Claims{  //Creation du JWT a partir des infos recuperees en BDD

        let iat1 = OffsetDateTime::now_utc();
        let exp1 = iat1 + Duration::hours(10);

        let iat = iat1
        .date()
        .with_hms_milli(iat1.hour(), iat1.minute(), iat1.second(), 0)
        .unwrap()
        .assume_utc();

        let exp = exp1
        .date()
        .with_hms_milli(exp1.hour(), exp1.minute(), exp1.second(), 0)
        .unwrap()
        .assume_utc();

        Claims {
            id : admin.id,
            name : admin.name.clone(),
            last_name : admin.last_name.clone(),
            mail : admin.mail.clone(),
            admin : true,
            change_password : admin.change,
            mfa_active : admin.otpactive,
            complete_authentication : verif,
            iat : iat,
            exp : exp,
        }
    }
     
 }

 pub fn hash_password(password : String) -> Result<String, ApiError> { //Fct pour hash a String
        
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    let password = argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;

    Ok(password)
}

pub fn chiffrer_password(password : String) -> Vec<u8> { //Fct pour chiffrer le password hashe


    let secret = env::var("KEY_BDD").expect("erreur chargement cle bdd");

    let nonce = env::var("NONCE").expect("erreur chargement du nonce");

    let key = Key::from_slice(secret.as_bytes()); // 32-bytes

    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(nonce.as_bytes()); // 96-bits 

    let ciphertext = cipher.encrypt(nonce, password.as_bytes().as_ref()).unwrap(); //chiffre le mot de passe qui est hashe

    ciphertext


}

 impl AdminInserable {

    pub fn from_admin_recu(admin : AdminRecu) -> AdminInserable { //Creation d un admin inserable

        AdminInserable {     //Renvoie la structure qui peut etre inseree en BDD
            name : admin.name,
            last_name : admin.last_name,
            mail : admin.mail,
            password : chiffrer_password(admin.password),
            change : false,
            otpactive : false

        }


    }
    
}


impl AdminEnvoye {

    pub fn from_admin(admin : Admin) -> AdminEnvoye{ //Fct pour renvoyer le nom et prenom dans le JSON lors de l'insertion de l'utilisateur


        AdminEnvoye { 
            id: admin.id, 
            name: admin.name, 
            last_name: admin.last_name, mail: admin.mail.to_string(), 
            change : admin.change, 
            otpactive : admin.optactive
        }
    }
    
}

impl Admin {


    pub fn verify_password(&self, password: &[u8]) -> Result<bool, ApiError> { //Verifier mot de passe de l'admin qui veut se connecter  

        //On va dechiffrer le mot de passe de la BDD
        //On va comparer les hash entre le mot de passe BDD et celui envoyé a l'api

        let secret = env::var("KEY_BDD").expect("erreur chargement cle bdd");
        let nonce = env::var("NONCE").expect("erreur chargement du nonce");

        let key = Key::from_slice(secret.as_bytes()); // 32-bytes
        let cipher = ChaCha20Poly1305::new(key);

        let nonce = Nonce::from_slice(nonce.as_bytes()); // 96-bits; unique per message

        let password_bdd = cipher.decrypt(nonce, self.password.as_ref()).unwrap();  //Dechiffre le hash du mot de passe

        let password_bdd = String::from_utf8(password_bdd).expect("Echec lecture"); //Transforme le mot de passe hashe en String pour comparer

        argon2::verify_encoded(password_bdd.as_str(), password)   //Comparaison des hashs
            .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
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

    //On va saler + hasher mot de passe
    //On recreer la variable en la passant en mutable pour ne
    //pas changer tout le code
    let mut admin = admin;

    admin.password = hash_password(admin.password)?;

    let admin = AdminInserable::from_admin_recu(admin);

    let admin = diesel::insert_into(admins::table)
        .values(admin)
        .get_result(& mut conn)?;

    let admin = AdminEnvoye::from_admin(admin);


    Ok(admin)
    }

    pub fn update_password(id: i32, cred: AdminChangeCred) -> Result<Self, ApiError> { //Mettre a jour donnees d un admin a partir de son id et JSON
        let mut conn = db::connection()?;

        let password = chiffrer_password(hash_password(cred.password)?);  //Hash + sel puis chiffrer mot de passe


        let admin = diesel::update(admins::table)
            .filter(admins::id.eq(id))
            .set((admins::password.eq(password), admins::change.eq(true)))  //modifie mot de passe en BDD
            .get_result(&mut conn)?;

        Ok(admin)
    }

    pub fn create_otp(id: i32, graine: String) -> Result<Self, ApiError> { //Ajouter en bdd la graine de l otp
        let mut conn = db::connection()?;


        let admin = diesel::update(admins::table)
            .filter(admins::id.eq(id))
            .set((admins::otp.eq(graine), admins::optactive.eq(true)))  //on ajoute la graine de l otp
            .get_result(&mut conn)?;

        Ok(admin)
    }

    pub fn verification_2fa(mail : String, code_verif: String) -> Result<AdminEnvoye, ApiError> { //verification de l'otp
        let mut conn = db::connection()?;


        let admin : Admin= admins::table
            .filter(admins::mail.eq(mail))
            .first(&mut conn)?;

        let otp = admin.otp.clone().unwrap();

        let auth = GoogleAuthenticator::new();

        if auth.verify_code(&otp, &code_verif, 1, 0) == true{

            Ok(AdminEnvoye::from_admin(admin))
        }

        else{
            Err(ApiError::new(403, "Interdit".to_string()))
        }
    

   
    }


    pub fn delete(id: i32) -> Result<usize, ApiError> { //Supprimer un admin de la BDD
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


mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }

}

pub fn verifier_session(session : &String) -> Option<Claims> { //Fct pour verifier valider du JWT

    let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

    let token_message = decode::<Claims>(&session, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256));

    match token_message{
        Ok(claim) => {
            let my_claims = claim.claims;

                if my_claims.admin ==true && my_claims.complete_authentication == true {
                    Some(my_claims)
                }
                else{
                    None
                }
            },
        _=> None
                    
    }
}


pub fn verifier_session_activer_2fa(session : &String) -> Option<Claims> { //Fct pour verifier le JWT pour activer le 2FA

    let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

    let token_message = decode::<Claims>(&session, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256));

    match token_message{
        Ok(claim) => {
            let my_claims = claim.claims;

                if my_claims.admin ==true && my_claims.mfa_active == false { //verifier que l otp n est pas deja activee
                    Some(my_claims)
                }
                else{
                    None
                }
            },
        _=> None
                    
    }
}

pub fn verifier_session_2fa(session : &Session) -> Option<Claims> { //Fct pour valider le JWT lors de la 2eme etape d authentification

    let session = session.get::<String>("claim");

    let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

    match session {

        Ok(data_session) => {

            match data_session {

                Some(data) =>{
                    let token_message = decode::<Claims>(&data, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256));

                    match token_message{
                        Ok(claim) => {
                            let my_claims = claim.claims;

                            if my_claims.admin ==true && my_claims.mfa_active == true {
                                Some(my_claims)
                            }
                            else{
                                None
                            }
                        },
                        _=> None
                    }

                },
                _ => None
            }


        },
        _ => None
    }
}