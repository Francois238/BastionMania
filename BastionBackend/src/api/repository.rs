use std::env;

//use crate::schema::to_user_config::publikey;
use crate::api_error::ApiError;
use crate::db;
use crate::entities::{Bastion, BastionInsertable, K8sRessource, K8sRessourceInsertable, Ressource, RessourceInsertable, SshRessource, SshRessourceInsertable, Users, UsersModification, WireguardRessource, WireguardRessourceInsertable};
use crate::model::{BastionModification, Claims, UsersInstanceCreate};
use crate::schema::users::wireguard;
use crate::schema::{bastion, k8sressource, ressource, sshressource, users, wireguardressource};
use actix_session::Session;
use actix_web::Result;
use diesel::associations::HasTable;
use diesel::dsl::Update;
use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use diesel::row::NamedRow;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use wireguard_keys;

impl Claims {
    pub fn verifier_session_admin(session: &Session) -> Option<Claims> {
        //Fct pour verifier valider du JWT

        let session = session.get::<String>("claim");

        let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

        match session {
            Ok(data_session) => match data_session {
                Some(data) => {
                    let token_message = decode::<Claims>(
                        &data,
                        &DecodingKey::from_secret(secret.as_ref()),
                        &Validation::new(Algorithm::HS256),
                    );

                    match token_message {
                        Ok(claim) => {
                            let my_claims = claim.claims;

                            if my_claims.admin == true
                                && my_claims.change_password == true
                                && my_claims.complete_authentication == true
                            {
                                Some(my_claims)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }

    pub fn verifier_session_user(session: &Session) -> Option<Claims> {
        //Fct pour verifier valider du JWT

        let session = session.get::<String>("claim");

        let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

        match session {
            Ok(data_session) => match data_session {
                Some(data) => {
                    let token_message = decode::<Claims>(
                        &data,
                        &DecodingKey::from_secret(secret.as_ref()),
                        &Validation::new(Algorithm::HS256),
                    );

                    match token_message {
                        Ok(claim) => {
                            let my_claims = claim.claims;

                            if my_claims.admin == false
                                && my_claims.change_password == true
                                && my_claims.complete_authentication == true
                            {
                                Some(my_claims)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl Bastion {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let un_bastion = bastion::table.load::<Bastion>(&mut conn)?;
        Ok(un_bastion)
    }

    pub fn create(bastion: BastionInsertable) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;
        let newbastion: Bastion = diesel::insert_into(bastion::table)
            .values(bastion)
            .get_result(&mut conn)?;
        Ok(newbastion)
    }

    // /bastion/{bastion_id} endpoint =================================================================

    pub fn find_un_bastion(id: i32) -> Result<Bastion, ApiError> {
        let mut conn = db::connection()?;

        let bastion = bastion::table.filter(bastion::id.eq(id)).first(&mut conn)?;

        Ok(bastion)
    }

    pub fn update_un_bastion(
        id: i32,
        modifications: BastionModification,
    ) -> Result<Bastion, ApiError> {
        let mut conn = db::connection()?;

        let name = modifications.name;
        let subnet_cidr = modifications.subnet_cidr;
        let agent_endpoint = modifications.agent_endpoint;

        let bastion = diesel::update(bastion::table)
            .filter(bastion::id.eq(id))
            .set((
                bastion::name.eq(name),
                bastion::subnet_cidr.eq(subnet_cidr),
                bastion::agent_endpoint.eq(agent_endpoint),
            ))
            .get_result(&mut conn)?;

        Ok(bastion)
    }

    pub fn delete_un_bastion(id: i32) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let bastion =
            diesel::delete(bastion::table.filter(bastion::id.eq(id))).execute(&mut conn)?;

        Ok(bastion)
    }

    pub fn verification_appartenance(user_id: i32, bastion_id: i32) -> Result<bool, ApiError> {
        let mut conn = db::connection()?;

        let users: Vec<Users> = users::table
            .filter(users::user_id.eq(user_id))
            .filter(users::bastion_id.eq(bastion_id))
            .load::<Users>(&mut conn)?;

        Ok(!users.is_empty())
    }

    pub fn bastion_user(user_id: i32) -> Result<Vec<Users>, ApiError> {
        let mut conn = db::connection()?;

        let users: Vec<Users> = users::table
            .filter(users::user_id.eq(user_id))
            .load::<Users>(&mut conn)?;

        Ok(users)
    }
}

// /bastion/{bastion_id}/users

impl Users {
    pub fn find_users_bastion(_bastion_id: i32) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_users = users::table
            .filter(users::bastion_id.eq(_bastion_id))
            .load::<Users>(&mut conn)?;
        Ok(des_users)
    }

    pub fn create_users(users: UsersModification) -> Result<Users, ApiError> {
        let mut conn = db::connection()?;
        let newusers: Users = diesel::insert_into(users::table)
            .values(users)
            .get_result(&mut conn)?;
        Ok(newusers)
    }

    // /bastion/{bastion_id}/users/{user_id} endpoint =================================================================

    pub fn find_un_user(bastion_id: i32, user_id: i32) -> Result<Users, ApiError> {
        let mut conn = db::connection()?;

        let user = users::table
            .filter(users::user_id.eq(user_id))
            .filter(users::bastion_id.eq(bastion_id))
            .first(&mut conn)?;

        Ok(user)
    }

    pub fn delete_all_users(bastion_id: i32) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let user = diesel::delete(users::table.filter(users::bastion_id.eq(bastion_id)))
            .execute(&mut conn)?;

        Ok(user)
    }

    pub fn delete_un_user(bastion_id: i32, user_id: i32) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let user = diesel::delete(
            users::table
                .filter(users::user_id.eq(user_id))
                .filter(users::bastion_id.eq(bastion_id)),
        )
        .execute(&mut conn)?;

        Ok(user)
    }
}

// /bastion/{bastion_id}/users/{user_id}/generate_wireguard

pub fn build_client_address(bastion_id: i32, user_id: i32) -> Result<String, ApiError> {
    let mut conn = db::connection()?;

    let bastion: Bastion = bastion::table
        .filter(bastion::id.eq(bastion_id))
        .first(&mut conn)?;

    let user: Users = users::table
        .filter(users::user_id.eq(user_id))
        .filter(users::bastion_id.eq(bastion_id))
        .first(&mut conn)?;

    let mut client_address = "10.10".to_string();
    client_address.push_str(".");
    client_address.push_str(bastion.net_id.clone().to_string().as_str());
    client_address.push_str(".");
    client_address.push_str(user.net_id.clone().to_string().as_str());
    Ok(client_address.to_string())
}

pub fn build_endpoint_user(bastion_ip: String, bastion_id: i32) -> Result<String, ApiError> {
    let mut conn = db::connection()?;

    let bastion: Bastion = bastion::table
        .filter(bastion::id.eq(bastion_id))
        .first(&mut conn)?;

    let mut endpoint_user = bastion_ip;
    endpoint_user.push_str(":");
    endpoint_user.push_str(bastion.port.clone().to_string().as_str());

    Ok(endpoint_user.to_string())
}

pub fn get_bastion_public_key(bastion_id: i32) -> Result<String, ApiError> {
    let mut conn = db::connection()?;

    let bastion: Bastion = bastion::table
        .filter(bastion::id.eq(bastion_id))
        .first(&mut conn)?;

    Ok(bastion.pubkey.clone())
}

pub fn get_bastion_subnet_cidr(bastion_id: i32) -> Result<String, ApiError> {
    let mut conn = db::connection()?;

    let bastion: Bastion = bastion::table
        .filter(bastion::id.eq(bastion_id))
        .first(&mut conn)?;

    Ok(bastion.subnet_cidr.clone())
}

pub fn update_un_user(user_id: i32, bool: bool) -> Result<Users, ApiError> {
    let mut conn = db::connection()?;

    let user = diesel::update(users::table)
        .filter(users::user_id.eq(user_id))
        .set((users::wireguard.eq(bool)))
        .get_result(&mut conn)?;

    Ok(user)
}

// /bastion/{bastion_id}/ressources        ===================================================================

impl Ressource {
    pub fn find_all_ressources(id_bastion: i32) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_ressources = ressource::table
            .filter(ressource::id_bastion.eq(id_bastion))
            .load::<Ressource>(&mut conn)?;
        Ok(des_ressources)
    }

    pub fn create_ressources(ressource: RessourceInsertable) -> Result<Ressource, ApiError> {
        let mut conn = db::connection()?;
        let newressource: Ressource = diesel::insert_into(ressource::table)
            .values(ressource)
            .get_result(&mut conn)?;
        Ok(newressource)
    }

    pub fn find_a_ressource(id: i32, id_bastion: i32) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let une_ressource = ressource::table
            .filter(ressource::id_bastion.eq(id_bastion))
            .filter(ressource::id.eq(id))
            .load::<Ressource>(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn delete_a_ressource(id: i32, id_bastion: i32) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;
        let ressource = diesel::delete(
            ressource::table
                .filter(ressource::id_bastion.eq(id_bastion))
                .filter(ressource::id.eq(id)),
        )
        .execute(&mut conn)?;
        Ok(ressource)
    }
}

impl WireguardRessource {
    pub fn find_all_wireguard_ressources(bastion_id: i32) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_ressources = wireguardressource::table
            .filter(wireguardressource::id_bastion.eq(bastion_id))
            .load::<WireguardRessource>(&mut conn)?;
        Ok(des_ressources)
    }

    pub fn find_a_wireguard_ressource(id: i32, bastion_id: i32) -> Result<WireguardRessource, ApiError>{
        let mut conn = db::connection()?;
        let une_ressource = wireguardressource::table
            .filter(wireguardressource::id_bastion.eq(bastion_id))
            .filter(wireguardressource::id.eq(id))
            .load::<WireguardRessource>(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn create_wireguard_ressources(ressource: WireguardRessourceInsertable) -> Result<WireguardRessource, ApiError> {
        let mut conn = db::connection()?;
        let newressource: WireguardRessource = diesel::insert_into(wireguardressource::table)
            .values(ressource)
            .get_result(&mut conn)?;
        Ok(newressource)
    }

    pub fn delete_a_wireguard_ressource(id: i32, bastion_id: i32) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;
        let ressource = diesel::delete(
            wireguardressource::table
                .filter(wireguardressource::id_bastion.eq(bastion_id))
                .filter(wireguardressource::id.eq(id)),
        )
            .execute(&mut conn)?;
        Ok(ressource)
    }
}

impl SshRessource{
    pub fn find_all_ssh_ressources(bastion_id: i32) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_ressources = sshressource::table
            .filter(sshressource::id_bastion.eq(bastion_id))
            .load::<SshRessource>(&mut conn)?;
        Ok(des_ressources)
    }

    pub fn find_a_ssh_ressource(id: i32, bastion_id: i32) -> Result<SshRessource, ApiError>{
        let mut conn = db::connection()?;
        let une_ressource = sshressource::table
            .filter(sshressource::id_bastion.eq(bastion_id))
            .filter(sshressource::id.eq(id))
            .load::<SshRessource>(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn create_ssh_ressources(ressource: SshRessourceInsertable) -> Result<SshRessource, ApiError> {
        let mut conn = db::connection()?;
        let newressource: SshRessource = diesel::insert_into(sshressource::table)
            .values(ressource)
            .get_result(&mut conn)?;
        Ok(newressource)
    }

    pub fn delete_a_ssh_ressource(id: i32, bastion_id: i32) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;
        let ressource = diesel::delete(
            sshressource::table
                .filter(sshressource::id_bastion.eq(bastion_id))
                .filter(sshressource::id.eq(id)),
        )
            .execute(&mut conn)?;
        Ok(ressource)
    }
}

impl K8sRessource{
    pub fn find_all_k8s_ressources(bastion_id: i32) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_ressources = k8sressource::table
            .filter(k8sressource::id_bastion.eq(bastion_id))
            .load::<K8sRessource>(&mut conn)?;
        Ok(des_ressources)
    }

    pub fn find_a_k8s_ressource(id: i32, bastion_id: i32) -> Result<K8sRessource, ApiError>{
        let mut conn = db::connection()?;
        let une_ressource = k8sressource::table
            .filter(k8sressource::id_bastion.eq(bastion_id))
            .filter(k8sressource::id.eq(id))
            .load::<K8sRessource>(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn create_k8s_ressources(ressource: K8sRessourceInsertable) -> Result<K8sRessource, ApiError> {
        let mut conn = db::connection()?;
        let newressource: K8sRessource = diesel::insert_into(k8sressource::table)
            .values(ressource)
            .get_result(&mut conn)?;
        Ok(newressource)
    }

    pub fn delete_a_k8s_ressource(id: i32, bastion_id: i32) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;
        let ressource = diesel::delete(
            k8sressource::table
                .filter(k8sressource::id_bastion.eq(bastion_id))
                .filter(k8sressource::id.eq(id)),
        )
            .execute(&mut conn)?;
        Ok(ressource)
    }
}
