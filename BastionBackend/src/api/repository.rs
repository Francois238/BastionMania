use crate::schema::{bastion, bastion_token, k8sressource, ressource, sshressource, users, wireguardressource};
use crate::api_error::ApiError;
use crate::db;
use crate::entities::{Bastion, BastionInsertable, BastionTokenInsertable, K8sRessource, K8sRessourceInsertable, Ressource, RessourceInsertable, SshRessource, SshRessourceInsertable, Users, UsersModification, WireguardRessource, WireguardRessourceInsertable, BastionToken};
use crate::model::{BastionModification};


use actix_web::Result;

use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;




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

    pub fn token_create(bastion_token: BastionTokenInsertable) -> Result<BastionToken, ApiError> {
        let mut conn = db::connection()?;
        let newtoken: BastionToken = diesel::insert_into(bastion_token::table)
            .values(bastion_token)
            .get_result(&mut conn)?;
        Ok(newtoken)
    }

    pub fn token_find(token: String) -> Result<BastionToken, ApiError> {
        let mut conn = db::connection()?;
        let un_bastion = bastion_token::table
            .filter(bastion_token::token.eq(token))
            .first::<BastionToken>(&mut conn)?;
        Ok(un_bastion)
    }

    pub fn token_delete(bastion_token: BastionToken) -> Result<(), ApiError> {
        let mut conn = db::connection()?;

        let bastion_id = bastion_token.bastion_id;

        let token = bastion_token.token;

        let _newtoken: usize = diesel::delete(bastion_token::table
            .filter(bastion_token::bastion_id.eq(bastion_id))
            .filter(bastion_token::token.eq(token)))
            .execute(&mut conn)?;            
        Ok(())
    }

    // /bastion/{bastion_id} endpoint =================================================================

    pub fn find_un_bastion(id: String) -> Result<Bastion, ApiError> {
        let mut conn = db::connection()?;

        let bastion = bastion::table.filter(bastion::bastion_id.eq(id)).first(&mut conn)?;

        Ok(bastion)
    }



    pub fn update_un_bastion(
        bastion_id: String,
        modifications: BastionModification,
    ) -> Result<Bastion, ApiError> {
        let mut conn = db::connection()?;

        let name = modifications.name;
        let subnet_cidr = modifications.subnet_cidr;
        let agent_endpoint = modifications.agent_endpoint;

        let bastion = diesel::update(bastion::table)
            .filter(bastion::bastion_id.eq(bastion_id))
            .set((
                bastion::name.eq(name),
                bastion::subnet_cidr.eq(subnet_cidr),
                bastion::agent_endpoint.eq(agent_endpoint),
            ))
            .get_result(&mut conn)?;

        Ok(bastion)
    }

    pub fn delete_un_bastion(id: String) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let bastion =
            diesel::delete(bastion::table.filter(bastion::bastion_id.eq(id))).execute(&mut conn)?;

        Ok(bastion)
    }

    pub fn verification_appartenance(user_id: String, ressource_id: String) -> Result<bool, ApiError> {
        let mut conn = db::connection()?;

        let users: Vec<Users> = users::table
            .filter(users::user_id.eq(user_id))
            .filter(users::ressource_id.eq(ressource_id))
            .load::<Users>(&mut conn)?;

        Ok(!users.is_empty())
    }

    pub fn bastion_user(user_id: String) -> Result<Vec<Users>, ApiError> {
        let mut conn = db::connection()?;

        let users: Vec<Users> = users::table
            .filter(users::user_id.eq(user_id))
            .load::<Users>(&mut conn)?;

        Ok(users)
    }
}

// /bastion/{bastion_id}/users

impl Users {
    pub fn find_users_bastion(ressource_id: String) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_users = users::table
            .filter(users::ressource_id.eq(ressource_id))
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

    pub fn find_un_user(ressource_id: String, user_id: String) -> Result<Users, ApiError> {
        let mut conn = db::connection()?;

        let user = users::table
            .filter(users::user_id.eq(user_id))
            .filter(users::ressource_id.eq(ressource_id))
            .first(&mut conn)?;

        Ok(user)
    }

    pub fn delete_all_users(ressource_id: String) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let user = diesel::delete(users::table.filter(users::ressource_id.eq(ressource_id)))
            .execute(&mut conn)?;

        Ok(user)
    }

    pub fn delete_un_user(ressource_id: String, user_id: String) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let user = diesel::delete(
            users::table
                .filter(users::user_id.eq(user_id))
                .filter(users::ressource_id.eq(ressource_id)),
        )
        .execute(&mut conn)?;

        Ok(user)
    }
}

// /bastion/{bastion_id}/users/{user_id}/generate_wireguard

pub fn build_client_address(ressource_id: String, user_id: String, bastion_id: String) -> Result<String, ApiError> {
    let mut conn = db::connection()?;

    let bastion: Bastion = bastion::table
        .filter(bastion::bastion_id.eq(bastion_id.clone()))
        .first(&mut conn)?;

    let user: Users = users::table
        .filter(users::user_id.eq(user_id))
        .filter(users::ressource_id.eq(ressource_id))
        .first(&mut conn)?;

    let mut client_address = "10.10".to_string();
    client_address.push_str(".");
    client_address.push_str(bastion.net_id.clone().to_string().as_str());
    client_address.push_str(".");
    client_address.push_str(user.net_id.clone().to_string().as_str());
    Ok(client_address.to_string())
}

pub fn build_endpoint_user(bastion_ip: String, bastion_id: String) -> Result<String, ApiError> {
    let mut conn = db::connection()?;

    let bastion: Bastion = bastion::table
        .filter(bastion::bastion_id.eq(bastion_id))
        .first(&mut conn)?;

    let mut endpoint_user = bastion_ip;
    endpoint_user.push_str(":");
    endpoint_user.push_str(bastion.port.clone().to_string().as_str());

    Ok(endpoint_user.to_string())
}

pub fn get_bastion_public_key(bastion_id: String) -> Result<String, ApiError> {
    let mut conn = db::connection()?;

    let bastion: Bastion = bastion::table
        .filter(bastion::bastion_id.eq(bastion_id))
        .first(&mut conn)?;

    Ok(bastion.pubkey)
}

pub fn get_bastion_subnet_cidr(bastion_id: String) -> Result<String, ApiError> {
    let mut conn = db::connection()?;

    let bastion: Bastion = bastion::table
        .filter(bastion::bastion_id.eq(bastion_id))
        .first(&mut conn)?;

    Ok(bastion.subnet_cidr)
}



// /bastion/{bastion_id}/ressources        ===================================================================

impl Ressource {
    pub fn find_all_ressources(id_bastion: String) -> Result<Vec<Self>, ApiError> {
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

    pub fn find_a_ressource(id_ressource: String) -> Result<Ressource, ApiError> {
        let mut conn = db::connection()?;
        let une_ressource = ressource::table
            .filter(ressource::id.eq(id_ressource))
            .first::<Ressource>(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn delete_a_ressource(id: String, id_bastion: String) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;
        let ressource = diesel::delete(
            ressource::table
                .filter(ressource::id.eq(id))
                .filter(ressource::id_bastion.eq(id_bastion))

        )
        .execute(&mut conn)?;
        Ok(ressource)
    }

    pub fn ressource_user(user_id: String) -> Result<Vec<Users>, ApiError> {
        let mut conn = db::connection()?;

        let users: Vec<Users> = users::table
            .filter(users::user_id.eq(user_id))
            .load::<Users>(&mut conn)?;

        Ok(users)
    }

    pub fn verification_appartenance(user_id: String, ressource_id: String) -> Result<bool, ApiError> {
        let mut conn = db::connection()?;

        let users: Vec<Users> = users::table
            .filter(users::user_id.eq(user_id))
            .filter(users::ressource_id.eq(ressource_id))
            .load::<Users>(&mut conn)?;

        Ok(!users.is_empty())
    }
}

impl WireguardRessource {
    pub fn find_all_wireguard_ressources(bastion_id: String) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_ressources = wireguardressource::table
            .filter(wireguardressource::id_bastion.eq(bastion_id))
            .load::<WireguardRessource>(&mut conn)?;
        Ok(des_ressources)
    }

    pub fn find_a_wireguard_ressource(id: i32, bastion_id: String) -> Result<WireguardRessource, ApiError>{
        let mut conn = db::connection()?;
        let une_ressource = wireguardressource::table
            .filter(wireguardressource::id.eq(id))
            .filter(wireguardressource::id_bastion.eq(bastion_id))
            .first::<WireguardRessource>(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn create_wireguard_ressources(ressource: WireguardRessourceInsertable) -> Result<WireguardRessource, ApiError> {
        let mut conn = db::connection()?;
        let newressource: WireguardRessource = diesel::insert_into(wireguardressource::table)
            .values(ressource)
            .get_result(&mut conn)?;
        Ok(newressource)
    }

    pub fn delete_a_wireguard_ressource(id: i32, bastion_id: String) -> Result<usize, ApiError> {
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
    pub fn find_all_ssh_ressources(bastion_id: String) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_ressources = sshressource::table
            .filter(sshressource::id_bastion.eq(bastion_id))
            .load::<SshRessource>(&mut conn)?;
        Ok(des_ressources)
    }

    pub fn find_a_ssh_ressource(id: i32, bastion_id: String) -> Result<SshRessource, ApiError>{
        let mut conn = db::connection()?;
        let une_ressource = sshressource::table
            .filter(sshressource::id_bastion.eq(bastion_id))
            .filter(sshressource::id.eq(id))
            .first::<SshRessource>(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn create_ssh_ressources(ressource: SshRessourceInsertable) -> Result<SshRessource, ApiError> {
        let mut conn = db::connection()?;
        let newressource: SshRessource = diesel::insert_into(sshressource::table)
            .values(ressource)
            .get_result(&mut conn)?;
        Ok(newressource)
    }

    pub fn delete_a_ssh_ressource(id: i32, bastion_id: String) -> Result<usize, ApiError> {
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
    pub fn find_all_k8s_ressources(bastion_id: String) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_ressources = k8sressource::table
            .filter(k8sressource::id_bastion.eq(bastion_id))
            .load::<K8sRessource>(&mut conn)?;
        Ok(des_ressources)
    }

    pub fn find_a_k8s_ressource(id: i32, bastion_id: String) -> Result<K8sRessource, ApiError>{
        let mut conn = db::connection()?;
        let une_ressource = k8sressource::table
            .filter(k8sressource::id_bastion.eq(bastion_id))
            .filter(k8sressource::id.eq(id))
            .first::<K8sRessource>(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn create_k8s_ressources(ressource: K8sRessourceInsertable) -> Result<K8sRessource, ApiError> {
        let mut conn = db::connection()?;
        let newressource: K8sRessource = diesel::insert_into(k8sressource::table)
            .values(ressource)
            .get_result(&mut conn)?;
        Ok(newressource)
    }

    pub fn delete_a_k8s_ressource(id: i32, bastion_id: String) -> Result<usize, ApiError> {
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
