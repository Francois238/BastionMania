use crate::api_error::ApiError;
use crate::db;
use crate::entities::userconfigssh::{UserConfigSsh, UserConfigSshInsertable};
use crate::entities::userconfigwireguard::{UserConfigWireguard, UserConfigWireguardInsertable};
use crate::entities::{
    Bastion, BastionInsertable, BastionToken, BastionTokenInsertable, K8sRessource,
    K8sRessourceInsertable, Ressource, RessourceInsertable, SshRessource, SshRessourceInsertable,
    Users, UsersModification, WireguardRessource, WireguardRessourceInsertable,
};
use crate::model::ressourcecredentialsssh::{ActivationSshSession, SSHPublicKey};
use crate::model::ressourcecredentialwireguard::{
    ActivationWireguardSession};

use crate::schema::{bastion, bastion_token, k8sressource, ressource, sshressource, user_config_ssh, user_config_wireguard, users, wireguardressource};

use actix_web::{HttpResponse, Result};

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
        log::debug!("Searching token: {}", token);

        let un_bastion = bastion_token::table
            .filter(bastion_token::token.eq(token))
            .first(&mut conn)?;

        Ok(un_bastion)
    }

    pub fn token_delete(bastion_token: BastionToken) -> Result<(), ApiError> {
        let mut conn = db::connection()?;

        let bastion_id = bastion_token.bastion_id;

        let token = bastion_token.token;

        let _newtoken: usize = diesel::delete(
            bastion_token::table
                .filter(bastion_token::bastion_id.eq(bastion_id))
                .filter(bastion_token::token.eq(token)),
        )
            .execute(&mut conn)?;
        Ok(())
    }

    // /bastion/{bastion_id} endpoint =================================================================

    pub fn find_un_bastion(id: String) -> Result<Bastion, ApiError> {
        let mut conn = db::connection()?;

        let bastion = bastion::table
            .filter(bastion::bastion_id.eq(id))
            .first(&mut conn)?;

        Ok(bastion)
    }

    pub fn delete_un_bastion(id: String) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let bastion =
            diesel::delete(bastion::table.filter(bastion::bastion_id.eq(id))).execute(&mut conn)?;

        Ok(bastion)
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
    pub fn find_users_ressources(ressource_id: String) -> Result<Vec<Self>, ApiError> {
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

pub fn build_client_address(
    ressource_id: String,
    user_id: String,
    bastion_id: String,
) -> Result<String, ApiError> {
    let mut conn = db::connection()?;

    let bastion: Bastion = bastion::table
        .filter(bastion::bastion_id.eq(bastion_id.clone()))
        .first(&mut conn)?;

    let user: Users = users::table
        .filter(users::user_id.eq(user_id))
        .filter(users::ressource_id.eq(ressource_id))
        .first(&mut conn)?;

    let client_ip = format!(
        "10.10.{}.{}",
        bastion.net_id.to_string(),
        user.net_id.to_string()
    );
    Ok(client_ip.to_string())
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
            .first(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn delete_a_ressource(id: String, id_bastion: String) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;
        let ressource = diesel::delete(
            ressource::table
                .filter(ressource::id.eq(id))
                .filter(ressource::id_bastion.eq(id_bastion)),
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

    pub fn ressource_bastion(ressource_id: String) -> Result<Bastion, ApiError> {
        let mut conn = db::connection()?;

        let ressource: Ressource = ressource::table
            .filter(ressource::id.eq(ressource_id))
            .first(&mut conn)?;

        let bastion: Bastion = bastion::table
            .filter(bastion::bastion_id.eq(ressource.id_bastion))
            .first(&mut conn)?;

        Ok(bastion)
    }

    pub fn verification_appartenance(
        user_id: String,
        ressource_id: String,
    ) -> Result<bool, ApiError> {
        let mut conn = db::connection()?;

        let users: Vec<Users> = users::table
            .filter(users::user_id.eq(user_id))
            .filter(users::ressource_id.eq(ressource_id))
            .load::<Users>(&mut conn)?;

        Ok(!users.is_empty())
    }

    pub fn delete_all_ressources(id_bastion: String) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;
        let ressource = diesel::delete(ressource::table.filter(ressource::id_bastion.eq(id_bastion)))
            .execute(&mut conn)?;
        Ok(ressource)
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

    pub fn find_a_wireguard_ressource(
        id: i32,
        bastion_id: String,
    ) -> Result<WireguardRessource, ApiError> {
        let mut conn = db::connection()?;
        let une_ressource = wireguardressource::table
            .filter(wireguardressource::id.eq(id))
            .filter(wireguardressource::id_bastion.eq(bastion_id))
            .first(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn create_wireguard_ressources(
        ressource: WireguardRessourceInsertable,
    ) -> Result<WireguardRessource, ApiError> {
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

impl SshRessource {
    pub fn find_all_ssh_ressources(bastion_id: String) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_ressources = sshressource::table
            .filter(sshressource::id_bastion.eq(bastion_id))
            .load::<SshRessource>(&mut conn)?;
        Ok(des_ressources)
    }

    pub fn find_a_ssh_ressource(id: i32, bastion_id: String) -> Result<SshRessource, ApiError> {
        let mut conn = db::connection()?;
        let une_ressource = sshressource::table
            .filter(sshressource::id_bastion.eq(bastion_id))
            .filter(sshressource::id.eq(id))
            .first(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn create_ssh_ressources(
        ressource: SshRessourceInsertable,
    ) -> Result<SshRessource, ApiError> {
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

impl K8sRessource {
    pub fn find_all_k8s_ressources(bastion_id: String) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let des_ressources = k8sressource::table
            .filter(k8sressource::id_bastion.eq(bastion_id))
            .load::<K8sRessource>(&mut conn)?;
        Ok(des_ressources)
    }

    pub fn find_a_k8s_ressource(id: i32, bastion_id: String) -> Result<K8sRessource, ApiError> {
        let mut conn = db::connection()?;
        let une_ressource = k8sressource::table
            .filter(k8sressource::id_bastion.eq(bastion_id))
            .filter(k8sressource::id.eq(id))
            .first(&mut conn)?;
        Ok(une_ressource)
    }

    pub fn create_k8s_ressources(
        ressource: K8sRessourceInsertable,
    ) -> Result<K8sRessource, ApiError> {
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

impl UserConfigWireguard {
    pub fn userconfigwireguardcreate(
        userconf: UserConfigWireguardInsertable,
    ) -> Result<(), ApiError> {
        let mut conn = db::connection()?;
        let _newuserconf: UserConfigWireguard = diesel::insert_into(user_config_wireguard::table)
            .values(userconf)
            .get_result(&mut conn)?;
        Ok(())
    }

    pub fn userconfigwireguardfind(
        user_id: String,
        ressource_id: String,
    ) -> Result<UserConfigWireguard, ApiError> {
        let mut conn = db::connection()?;
        let userconf = user_config_wireguard::table
            .filter(user_config_wireguard::uuid_user.eq(user_id))
            .filter(user_config_wireguard::uuid_ressource.eq(ressource_id))
            .first(&mut conn)?;
        Ok(userconf)
    }

    pub fn userconfigwireguarddelete(
        user_id: String,
        ressource_id: String,
    ) -> Result<(), ApiError> {
        let mut conn = db::connection()?;
        let _userconf = diesel::delete(
            user_config_wireguard::table
                .filter(user_config_wireguard::uuid_user.eq(user_id))
                .filter(user_config_wireguard::uuid_ressource.eq(ressource_id)),
        )
            .execute(&mut conn)?;
        Ok(())
    }

    pub fn userconfigwireguarddeleteall(ressource_id: String) -> Result<(), ApiError> {
        let mut conn = db::connection()?;
        let _userconf = diesel::delete(
            user_config_wireguard::table
                .filter(user_config_wireguard::uuid_ressource.eq(ressource_id)),
        )
            .execute(&mut conn)?;
        Ok(())
    }

    pub async fn start_wireguard_session(
        user_id: String,
        ressource_id: String,
    ) -> Result<(), ApiError> {
        log::info!("start_wireguard_session");
        let userconfig =
            UserConfigWireguard::userconfigwireguardfind(user_id.clone(), ressource_id.clone())?;
        let client = reqwest::Client::new();
        let bastion = Ressource::ressource_bastion(ressource_id.clone())?;
        let ip = format!(
            "10.10.{}.{}",
            bastion.net_id.to_string(),
            userconfig.user_net_id.to_string()
        );
        log::debug!("ip: {}", ip);

        let session = ActivationWireguardSession {
            id: userconfig.uuid_ressource.clone(),
            client_id: user_id.clone(),
            public_key: userconfig.pubkey.clone(),
            client_ip: ip.clone(),
            target_ip: bastion.subnet_cidr.clone(),
        };
        //TODO url
        let url = format!(
            "http://bastion-internal-{}:9000/wireguard/configs",
            bastion.bastion_id.clone()
        );

        let _response = client
            .post(&url)
            .json(&session)
            .send()
            .await
            .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;
        log::debug!("response: {:?}", _response);

        Ok(())
    }

    pub async fn stop_wireguard_session(
        user_id: String,
        ressource_id: String,
    ) -> Result<(), ApiError> {
        log::info!("stop_wireguard_session");
        let _userconfig =
            UserConfigWireguard::userconfigwireguardfind(user_id.clone(), ressource_id.clone())?;
        let client = reqwest::Client::new();
        let bastion = Ressource::ressource_bastion(ressource_id.clone())?;

        //TODO url
        let url = format!(
            "http://bastion-internal-{}:9000/wireguard/configs/{ressource_id}/{user_id}",
            bastion.bastion_id.clone()
        );

        let _response = client
            .delete(&url)
            .send()
            .await
            .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;
        log::debug!("response: {:?}", _response);

        Ok(())
    }
}

impl UserConfigSsh {
    pub fn userconfigsshcreate(userconf: UserConfigSshInsertable) -> Result<(), ApiError> {
        let mut conn = db::connection()?;
        let _newuserconf: UserConfigSsh = diesel::insert_into(user_config_ssh::table)
            .values(userconf)
            .get_result(&mut conn)?;
        Ok(())
    }

    pub fn userconfigsshfind(
        user_id: String,
        ressource_id: String,
    ) -> Result<UserConfigSsh, ApiError> {
        let mut conn = db::connection()?;
        let userconf = user_config_ssh::table
            .filter(user_config_ssh::uuid_user.eq(user_id))
            .filter(user_config_ssh::uuid_ressource.eq(ressource_id))
            .first(&mut conn)?;
        Ok(userconf)
    }

    pub fn userconfigsshdelete(user_id: String, ressource_id: String) -> Result<(), ApiError> {
        let mut conn = db::connection()?;
        let _userconf = diesel::delete(
            user_config_ssh::table
                .filter(user_config_ssh::uuid_user.eq(user_id))
                .filter(user_config_ssh::uuid_ressource.eq(ressource_id)),
        )
            .execute(&mut conn)?;
        Ok(())
    }

    pub fn userconfigsshdeleteall(ressource_id: String) -> Result<(), ApiError> {
        let mut conn = db::connection()?;
        let _userconf = diesel::delete(
            user_config_ssh::table.filter(user_config_ssh::uuid_ressource.eq(ressource_id)),
        )
            .execute(&mut conn)?;
        Ok(())
    }

    pub async fn start_ssh_session(user_id: String, ressource_id: String) -> Result<(), ApiError> {
        log::info!("start_ssh_session");
        let client = reqwest::Client::new();
        let userconfig = UserConfigSsh::userconfigsshfind(user_id.clone(), ressource_id.clone())?;
        log::debug!("userconfig: {:?}", userconfig);
        let ressource = Ressource::find_a_ressource(ressource_id.clone())?;
        log::debug!("ressource: {:?}", ressource);
        let bastion_id = ressource.id_bastion.clone();
        let sshressource: SshRessource = SshRessource::find_a_ssh_ressource(
            ressource
                .id_ssh
                .clone()
                .ok_or(ApiError::new(404, "Not Found".to_string()))?,
            bastion_id.clone(),
        )?;
        log::debug!("sshressource: {:?}", sshressource);
        let session = ActivationSshSession {
            id: user_id.clone(),
            name: userconfig.username.clone(),
            public_key: SSHPublicKey::from_string(userconfig.pubkey.clone()),
        };
        let resource_name = ressource.name.clone();
        let url = format!("http://bastion-internal-{bastion_id}:9000/ssh/ressources/{resource_name}/users");

        let _response = client
            .post(&url)
            .json(&session)
            .send()
            .await
            .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;
        log::debug!("response: {:?}", _response);

        return Ok(());
    }

    pub async fn stop_ssh_session(user_id: String, ressource_id: String) -> Result<(), ApiError> {
        log::info!("stop_ssh_session");
        let _userconfig = UserConfigSsh::userconfigsshfind(user_id.clone(), ressource_id.clone())?;
        let ressource = Ressource::find_a_ressource(ressource_id.clone())?;
        let bastion_id = ressource.id_bastion.clone();

        //TODO url
        let client = reqwest::Client::new();
        let url = format!("http://bastion-internal-{bastion_id}:9000/ssh/ressources/{ressource_id}/users/{user_id}");
        let _res = client
            .delete(&url)
            .send()
            .await
            .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;
        log::debug!("response: {:?}", _res);

        return Ok(());
    }
}

pub async fn user_suppression(
    user_id: String,
    ressource_id: String,
) -> Result<HttpResponse, ApiError> {
    log::info!("user_suppression");
    let ressource = Ressource::find_a_ressource(ressource_id.clone())?;

    if ressource.rtype == "wireguard" && ressource.id_wireguard.is_some() {
        log::debug!("wireguard: {}", ressource.id_wireguard.clone().unwrap());

        let _ = UserConfigWireguard::stop_wireguard_session(user_id.clone(), ressource_id.clone())
            .await?;
        let _ =
            UserConfigWireguard::userconfigwireguarddelete(user_id.clone(), ressource_id.clone())?;
    } else if ressource.rtype == "ssh" && ressource.id_ssh.is_some() {
        log::debug!("ssh: {}", ressource.id_ssh.clone().unwrap());

        let _ = UserConfigSsh::stop_ssh_session(user_id.clone(), ressource_id.clone()).await?;
        let _ = UserConfigSsh::userconfigsshdelete(user_id.clone(), ressource_id.clone())?;
    }

    // supprimer le user de la ressource
    let user_suppr = Users::delete_un_user(ressource_id.clone(), user_id.clone())?;
    Ok(HttpResponse::Ok().json(user_suppr))
}

pub async fn ressource_suppression(
    bastion_id: String,
    ressource_id: String,
) -> Result<HttpResponse, ApiError> {
    log::info!("ressource_suppression");
    let ressource = Ressource::find_a_ressource(ressource_id.clone())?;
    let users: Vec<Users> = Users::find_users_ressources(ressource.id.clone())?;
    let client = reqwest::Client::new();

    for user in users {
        let _ = user_suppression(user.user_id, ressource_id.clone()).await?;
    }
    //TODO envoyer à bastion

    // supprimer la ressource de la base de donnée
    let rtype = ressource.rtype;
    log::debug!("rtype: {}", rtype);

    if rtype == "wireguard" && ressource.id_wireguard.is_some() {
        let wid = ressource
            .id_wireguard
            .ok_or(ApiError::new(404, "Not Found".to_string()))?
            .clone();
        let _ = WireguardRessource::delete_a_wireguard_ressource(wid, bastion_id.clone())?;
    } else if rtype == "ssh" && ressource.id_ssh.is_some() {
        let sid = ressource
            .id_ssh
            .ok_or(ApiError::new(404, "Not Found".to_string()))?
            .clone();
        let _ = SshRessource::delete_a_ssh_ressource(sid, bastion_id.clone())?;

        let url = format!("http://bastion-internal-{bastion_id}:9000/ssh/ressources/{ressource_id}");
        let _response = client
            .delete(&url)
            .send()
            .await
            .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;
        log::debug!("response: {:?}", _response);
    }
    let ressource = Ressource::delete_a_ressource(ressource_id, bastion_id)?;
    Ok(HttpResponse::Ok().json(ressource))
}

pub async fn suppression_bastion(bastion_id: String) -> Result<HttpResponse, ApiError> {
    log::info!("suppression_bastion");

    let ressources: Vec<Ressource> = Ressource::find_all_ressources(bastion_id.clone())?;

    for ressource in ressources {
        let users: Vec<Users> = Users::find_users_ressources(ressource.id.clone())?;
        for user in users {
            let _ = user_suppression(user.user_id.clone(), ressource.id.clone()).await?;
            let _ = UserConfigSsh::userconfigsshdelete(user.user_id.clone(), ressource.id.clone())?;
            let _ = UserConfigWireguard::userconfigwireguarddelete(user.user_id.clone(), ressource.id.clone())?;
        }
        let rtype = ressource.rtype;
        if rtype == "wireguard" {
            let specid = ressource.id_wireguard.unwrap();
            let WireguardRessource = WireguardRessource::delete_a_wireguard_ressource(specid, bastion_id.clone())?;
        } else if rtype == "ssh" {
            let specid = ressource.id_ssh.unwrap();
            let SshRessource = SshRessource::delete_a_ssh_ressource(specid, bastion_id.clone())?;
        }
        let _ = Users::delete_all_users(ressource.id.clone())?;
    }

    let _ = Ressource::delete_all_ressources(bastion_id.clone())?;
    let bastion = Bastion::delete_un_bastion(bastion_id.clone())?;
    Ok(HttpResponse::Ok().json(bastion))
}
