
use std::env;

use crate::schema::serveur_config::publikey;
use crate::{api_error::ApiError, schema::serveur_config};
use crate::db;
use crate::schema::bastion;
use actix_web::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};


#[derive(Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = bastion)]
pub struct BastionCreation{
    pub name: String,
    pub protocols: String,
    pub wireguard_id: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct BastionModification { 
    pub nom:String,
    pub protocols:String
}

#[derive(Queryable)]
#[derive(Serialize)]
pub struct Bastion{
    pub id: i32,
    pub name: String,
    pub protocols: String,
    pub wireguard_id: Option<i32>
}

#[derive(Queryable)]
#[derive(Serialize)]
pub struct WireguardClientConf{
    pub privatekey: String,
    pub address: String,
    pub peerpublickey: String,
    pub peerallowedips: String,
    pub peerendpoint: String
}

#[derive(Queryable)]
#[derive(Serialize)]
pub struct WireguardServerConf{
    pub id: i32,
    pub publikey: String,
    pub presharedkey: String,
    pub ip: String
}

#[derive(Serialize, Deserialize)]
pub struct WireguardServerModification { 
    pub ip: String
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
    

 // /bastion endpoint

pub fn find_all() -> Result<Vec<Bastion>, ApiError> {  //Fct pour récuperer tous les admins de la BDD
    let mut conn = db::connection()?;

    let admins = bastion::table
        .load::<Bastion>(&mut conn)?; //On recupere la liste des noms

    

    Ok(admins)
}

    pub fn create(bastion: BastionCreation) -> Result<Bastion, ApiError> { //Fct pour créer un admin à partir du JSON envoyé a l'api
    let mut conn = db::connection()?;

    //On va saler + hasher mot de passe
    //On recreer la variable en la passant en mutable pour ne
    //pas changer tout le code
    let mut bastion = bastion;

    let bastion = diesel::insert_into(bastion::table)
        .values(bastion)
        .get_result(& mut conn)?;


    Ok(bastion)
}


// /bastion/{bastion_id} endpoint

pub fn find_un_bastion(id: i32) -> Result<Bastion, ApiError> { 

    let mut conn = db::connection()?;

    let bastion = bastion::table
        .filter(bastion::id.eq(id))
        .first(&mut conn)?;

    Ok(bastion)
}

pub fn update_un_bastion(id: i32, modifications: BastionModification) -> Result<Bastion, ApiError> { 
    let mut conn = db::connection()?;

    let nom = modifications.nom;
    let protocols= modifications.protocols;

    let bastion = diesel::update(bastion::table)
        .filter(bastion::id.eq(id))
        .set((bastion::name.eq(nom), bastion::protocols.eq(protocols)))
        .get_result(&mut conn)?;

    Ok(bastion)
}

pub fn delete_un_bastion(id: i32) -> Result<usize, ApiError> { 
    let mut conn = db::connection()?;

    let bastion = diesel::delete(
        bastion::table
                .filter(bastion::id.eq(id))
        )
        .execute(&mut conn)?;

    Ok(bastion)
}



// /bastion/{bastion_id}/wireguard

pub fn get_server_config(id: i32) -> Result<WireguardServerConf, ApiError> { 

    let mut conn = db::connection()?;

    let form = serveur_config::table
        .filter(serveur_config::id.eq(id))
        .first(&mut conn)?;

    Ok(form)
}

pub fn patch_server_config(id: i32, modifications: WireguardServerModification) -> Result<WireguardServerConf, ApiError> { 
    let mut conn = db::connection()?;

    let ip = modifications.ip;

    let form = diesel::update(serveur_config::table)
        .filter(serveur_config::id.eq(id))
        .set((serveur_config::ip.eq(ip)))
        .get_result(&mut conn)?;

    Ok(form)
}





//fin de fichier

/*
pub fn verifier_session_admin(session : &Session) -> Option<Claims> { //Fct pour verifier valider du JWT

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

                            if my_claims.admin ==true  && my_claims.change_password == true &&  my_claims.complete_authentication==true{
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

pub fn verifier_session_user(session : &Session) -> Option<Claims> { //Fct pour verifier valider du JWT

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

                            if my_claims.admin ==false  && my_claims.change_password == true &&  my_claims.complete_authentication==true{
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
*/
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