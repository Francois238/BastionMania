
use std::env;

//use crate::schema::to_user_config::publikey;
use crate::{api_error::ApiError};
use crate::db;
use crate::schema::{bastion, users};
use crate::model::{BastionModification};
use crate::entities::{Bastion, BastionInsertable, Users, UsersModification};
use diesel::query_dsl::RunQueryDsl;
use wireguard_keys;
use actix_web::Result;
use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};



impl Bastion {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let un_bastion = bastion::table
            .load::<Bastion>(&mut conn)?;
        Ok(un_bastion)
    }

    pub fn create(bastion: BastionInsertable) -> Result<Bastion, ApiError> {
        let mut conn = db::connection()?;
        let newbastion: Bastion = diesel::insert_into(bastion::table)
            .values(bastion)
            .get_result(&mut conn)?;
        Ok(newbastion)
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

        let name = modifications.name;
        let subnet_cidr = modifications.subnet_cidr;
        let agent_endpoint = modifications.agent_endpoint;

        let bastion = diesel::update(bastion::table)
            .filter(bastion::id.eq(id))
            .set((bastion::name.eq(name),
                  bastion::subnet_cidr.eq(subnet_cidr),
                  bastion::agent_endpoint.eq(agent_endpoint)))
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
}

// /bastion/{bastion_id}/users

impl Users{
    pub fn find_users_bastion(_bastion_id: i32) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_users = users::table
            .filter(users::bastion_id.eq(_bastion_id))
            .load::<Users>(&mut conn)?;
        Ok(des_users)

    }

    pub fn create_users(users: Users) -> Result<Users, ApiError> {
        let mut conn = db::connection()?;
        let newusers: Users = diesel::insert_into(users::table)
            .values(users)
            .get_result(&mut conn)?;
        Ok(newusers)
    }

    // /bastion/{bastion_id}/users/{user_id} endpoint =================================================================

    pub fn find_un_user(id: i32) -> Result<Users, ApiError> {
        let mut conn = db::connection()?;

        let user = users::table
            .filter(users::id.eq(id))
            .first(&mut conn)?;

        Ok(user)
    }

    pub fn delete_un_user(id: i32) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let user = diesel::delete(
            users::table
                .filter(users::id.eq(id))
        )
            .execute(&mut conn)?;

        Ok(user)
    }

}


// /bastion/{bastion_id}/wireguard ================================================================
/*
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

}*/