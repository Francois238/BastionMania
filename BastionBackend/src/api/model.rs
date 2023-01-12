
use std::env;

//use crate::schema::to_user_config::publikey;
use crate::{api_error::ApiError};
use crate::db;
use crate::schema::{bastion, to_user_config, to_agent_config, agent};
use wireguard_keys;
use actix_web::Result;
use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};



//BASTION ===========================================================
#[derive(Queryable)]
#[derive(Serialize)]
pub struct Bastion{
    id: i32,
    name: String,
    protocols: String,
    subnet_CIDR: String,
    endpoint_ip: String,
    endpoint_port: String,
    to_agent_config_id: String,
    to_client_config_id: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = bastion)]
pub struct BastionModification { 
    pub name:String,
    pub protocols:String,
    pub subnet_cidr: String,
    pub endpoint_ip: String,
    pub endpoint_port: String,
}


//Wireguard côté user ========================================================

#[derive(Queryable)]
#[derive(Serialize)]
pub struct WireguardUserConf{
    pub id: i32,
    pub publickey: String,
    pub privatekey: String,
    pub ip: String
}

#[derive(Serialize, Deserialize)]
pub struct WireguardUserConfModification {
    pub publickey: String,
    pub privatekey: String,
    pub ip: String
}

#[derive(Serialize, Deserialize)]
pub struct BastionUserKeyGeneration{
    pub publickey: String,
    pub privatekey: String,
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

// Wireguard côté agent ==============================================================

#[derive(Queryable)]
#[derive(Serialize)]
pub struct WireguardAgentConf{
    pub id: i32,
    pub privatekey: String,
    pub publickey: String,
    pub address: String,
    pub peerallowedips: String,
    pub peerendpoint: String
}


#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = to_agent_config)]
pub struct WireguardAgentConfModification{
    pub privatekey: String,
    pub publickey: String,
    pub address: String,
    pub peerallowedips: String,
    pub peerendpoint: String
}

#[derive(Serialize, Deserialize)]
pub struct BastionAgentKeyGeneration{
    pub publickey: String,
    pub privatekey: String,
}

#[derive(Serialize, Deserialize)]
pub struct AgentKeyGeneration{
    pub publickey: String,
    pub privatekey: String,
}
 // /bastion endpoint =============================================================================

pub fn find_all() -> Result<Vec<Bastion>, ApiError> {
    let mut conn = db::connection()?;

    let bastion = bastion::table
        .load::<Bastion>(&mut conn)?;
    Ok(bastion)
}

pub fn create(bastion: BastionModification) -> Result<Bastion, ApiError> {
    let mut conn = db::connection()?;
    let mut bastion = bastion;
    let newbastion: BastionModification = diesel::insert_into(bastion::table)
        .values(bastion)
        .get_result(& mut conn)?;

    let to_agent_priv = wireguard_keys::Privkey::generate();
    let to_agent_pub = to_agent_priv.pubkey();

    let donnee_agent = WireguardAgentConfModification{
        privatekey: to_agent_priv.to_string(),
        publickey: to_agent_pub.to_string(),
        address: "10.10.1.2".to_string(),
        peerallowedips: newbastion.cidr,
        peerendpoint: newbastion.endpoint_ip
    };


    let to_agent_form  : WireguardAgentConfModification = diesel::insert_into( to_agent_config::table).values( donnee_agent).get_result(& mut conn)?;

    let to_user_priv = wireguard_keys::Privkey::generate();
    let to_user_pub = to_user_priv.pubkey();

    let agent_priv = wireguard_keys::Privkey::generate();
    let agent_pub = agent_priv.pubkey();
    
    Ok(bastion)


    // retourne config_agent(privatekeyagent, address_agent, pubkeybastion, address_bastion)
}


// /bastion/{bastion_id} endpoint =================================================================

pub fn find_un_bastion(id: i32) -> Result<Bastion, ApiError> { 

    let mut conn = db::connection()?;

    let bastion = bastion::table
        .filter(bastion::id.eq(id))
        .first(&mut conn)?;

    Ok(bastion)
}

pub fn update_un_bastion(id: i32, modifications: BastionModification) -> Result<Bastion, ApiError> { 
    let mut conn = db::connection()?;

    let nom = modifications.name;
    let protocols= modifications.protocols;
    let subnet_CIDR = modifications.subnet_cidr;
    let endpoint_ip = modifications.endpoint_ip;
    let endpoint_port = modifications.endpoint_port;

    let bastion = diesel::update(bastion::table)
        .filter(bastion::id.eq(id))
        .set((bastion::name.eq(nom),
                bastion::protocols.eq(protocols),
                bastion::subnet_cidr.eq(subnet_CIDR),
                bastion::endpoint_port.eq(endpoint_port),
                bastion::endpoint_ip.eq(endpoint_ip)))
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



// /bastion/{bastion_id}/wireguard ================================================================

pub fn get_user_config(id: i32) -> Result<WireguardUserConf, ApiError> {

    let mut conn = db::connection()?;

    let form = to_user_config::table
        .filter(to_user_config::id.eq(id))
        .first(&mut conn)?;

    Ok(form)
}

pub fn patch_user_config(id: i32, modifications: WireguardUserConfModification) -> Result<WireguardUserConf, ApiError> {
    let mut conn = db::connection()?;

    let ip = modifications.ip;

    let form = diesel::update(to_user_config::table)
        .filter(to_user_config::id.eq(id))
        .set((to_user_config::ip.eq(ip)))
        .get_result(&mut conn)?;

    Ok(form)
}

// /bastion/{bastion_id}/user =====================================================================
// /bastion/{bastion_id}/user/{user_id} ===========================================================
// /bastion/{bastion_id}//user/{user_id}/wireguard ================================================



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