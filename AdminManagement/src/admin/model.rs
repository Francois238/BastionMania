
use std::env;

use crate::api_error::ApiError;
use crate::db;
use crate::schema::admins;
use actix_session::Session;
use actix_web::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use argon2::Config;
use rand::Rng;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce}; // Or `XChaCha20Poly1305`
use chacha20poly1305::aead::{Aead, NewAead};
use time::{OffsetDateTime};
use jsonwebtoken::{decode,  DecodingKey, Validation, Algorithm};


#[derive(Serialize, Deserialize)]
pub struct CodeOtp{ //Structure envoye dans le JSON
    pub url: String
}

//Structure gestion des admins

#[derive(Serialize, Deserialize)]
pub struct AdminMessage { //Structure envoye dans le JSON
    pub name: String,
    pub last_name :String,
    pub mail : String,
    pub password: String,
}

#[derive(AsChangeset,Insertable)]
#[diesel(table_name = admins)]
pub struct AdminInserable { //Structure inseree en BDD pour ajouter un admin
    pub name: String,
    pub last_name :String,
    pub mail : String,
    pub password: Vec<u8>,
}

#[derive( Queryable)]
#[derive(Serialize)]
pub struct Admin { //Structure recupere par requete BDD
    pub id: i32,
    pub name: String,
    pub last_name :String,
    pub mail : String,
    #[serde(skip_serializing)]
    pub password: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminChangeCred { //Structure envoye dans le JSON pour changer password
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {  //Structure composant le JWT
    pub id : i32,
    pub name: String,
    pub last_name :String,
    pub mail : String,
    pub admin: bool,
    pub change_password : bool,
    pub mfa_active : bool,
    pub complete_authentication : bool,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
 }



pub fn hash_password(password : String) -> Result<String, ApiError> {
        
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        let password = argon2::hash_encoded(password.as_bytes(), &salt, &config)
            .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(password)
}

pub fn chiffrer_password(password : String) -> Vec<u8> {


    let secret = env::var("KEY_BDD").expect("erreur chargement cle bdd");

    let nonce = env::var("NONCE").expect("erreur chargement du nonce");

    let key = Key::from_slice(secret.as_bytes()); // 32-bytes

    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(nonce.as_bytes()); // 96-bits 

    let ciphertext = cipher.encrypt(nonce, password.as_bytes().as_ref()).unwrap(); //chiffre le mot de passe qui est hashe

    ciphertext


}

    


impl AdminInserable {

    pub fn from_admin_message(admin : AdminMessage) -> AdminInserable { //Creation d un admin inserable


        AdminInserable {     //Renvoie la structure qui peut etre inseree en BDD
            name : admin.name,
            last_name : admin.last_name,
            mail : admin.mail,
            password : chiffrer_password(admin.password),
        }


    }
    
}


impl Admin {


    pub fn find_all() -> Result<Vec<Self>, ApiError> {  //Fct pour récuperer tous les admins de la BDD
        let mut conn = db::connection()?;

        let admins = admins::table
            .load::<Admin>(&mut conn)?; //On recupere la liste des noms

        

        Ok(admins)
    }

    pub fn find(id: i32) -> Result<Self, ApiError> { //Fct pour recuperer 1 admin en particulier de la BDD

        let mut conn = db::connection()?;

        let admin = admins::table
            .filter(admins::id.eq(id))
            .first(&mut conn)?;

        Ok(admin)
    }
    

    pub fn create(admin: AdminMessage) -> Result<Admin, ApiError> { //Fct pour créer un admin à partir du JSON envoyé a l'api
        let mut conn = db::connection()?;

        //On va saler + hasher mot de passe
        //On recreer la variable en la passant en mutable pour ne
        //pas changer tout le code
        let mut admin = admin;

        admin.password = hash_password(admin.password)?;

        let admin = AdminInserable::from_admin_message(admin);

        let admin = diesel::insert_into(admins::table)
            .values(admin)
            .get_result(& mut conn)?;


        Ok(admin)
    }


    pub fn update_password(id: i32, cred: AdminChangeCred) -> Result<Self, ApiError> { //Mettre a jour donnees d un admin a partir de son id et JSON
        let mut conn = db::connection()?;

        let password = chiffrer_password(hash_password(cred.password)?);  //Hash + sel puis chiffrer mot de passe


        let admin = diesel::update(admins::table)
            .filter(admins::id.eq(id))
            .set(admins::password.eq(password))  //modifie mot de passe en BDD
            .get_result(&mut conn)?;

        Ok(admin)
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

pub fn premiere_utilisation_bastion(admin : AdminMessage) -> Result<Admin, ApiError>  { //Fct pour creer l'admin par défaut 

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

pub fn verifier_session(session : &Session) -> Option<Claims> { //Fct pour verifier valider du JWT

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

                            if my_claims.admin ==true && my_claims.change_password == true &&  my_claims.complete_authentication==true{
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


pub fn verifier_session_simple(session : &Session) -> Option<Claims> { //Fct pour verifier valider du JWT

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

                            if my_claims.admin ==true{
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