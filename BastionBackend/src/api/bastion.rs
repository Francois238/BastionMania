use actix_web::{
    delete,
    get,
    patch, post, web, HttpResponse, HttpRequest,
};
use base64::{engine, Engine};
use rand::{SeedableRng, RngCore};
use std::env;

use crate::{api::*, model::{claims::{VerifyAdmin, VerifyUser}, ressourcecredentialsssh::{RessourceCredentialsSsh, ConfigSshInstanceCreate, ActivationSshSession}, ressourcecredentialwireguard::{ActivationWireguardSession, ConfigWireguardInstanceCreate, RessourceCredentialsWireguard}, ressourcemodification::{RessourceSshCreation, RessourceWireguardCreation}, agentproof::AgentPairInfo}, entities::{userconfigssh::{UserConfigSshInsertable, UserConfigSsh}, userconfigwireguard::{UserConfigWireguardInsertable, UserConfigWireguard, self}, ressource}};
use crate::api_error::ApiError;
use crate::services::{generate_bastion_freenetid, generate_bastion_freeport,generate_user_freenetid};
//use derive_more::{Display};
use crate::entities::{Bastion, BastionInsertable, BastionTokenInsertable, K8sRessource, K8sRessourceInsertable, Ressource, RessourceInsertable, SshRessource, SshRessourceInsertable, Users, UsersModification, WireguardRessource, WireguardRessourceInsertable};
use crate::model::{
    BastionInstanceCreate, BastionModification, BastionSuppression, ConfigAgent,
    ConfigClient, ConfigUser, InstanceClient, RetourAPI, UsersCreation};
use log::error;
use repository::*;
use tracing::info;
use crate::model::k8sressourcemodification::K8sRessourceCreation;
use crate::model::ressourcemodification::RessourceCreation;
use crate::model::sshressourcemodification::SshRessourceCreation;
use crate::model::wireguardressourcemodification::WireguardRessourceCreation;
use uuid::Uuid;
use crate::model::agentproof::AgentAskPairInfo;

#[post("/agent")]
pub async fn Config_my_agent(
    agent_ask_info: web::Json<AgentAskPairInfo>,
) -> Result<HttpResponse, ApiError> {
    let agent_ask_info = agent_ask_info.into_inner();
    let my_bastion=Bastion::token_find(agent_ask_info.token)?;
    let bastion = Bastion::find_un_bastion(my_bastion.bastion_id.to_string())?;

    let agent_pair = AgentPairInfo{
        agent_host: agent_ask_info.agent_host,
        public_key: agent_ask_info.public_key,
        target_cidr: bastion.subnet_cidr,
    };

    let client = reqwest::Client::new();
    let url = format!("http://bastion-internal-{}:9000/agent", my_bastion.bastion_id.to_string());
    let res = client
        .post(&url)
        .json(&agent_pair)
        .send()
        .await
        .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

    let _suppr = Bastion::token_delete(my_bastion)?;
    let bastion_public_key = res.text().await.map_err(|e| {
        log::error!("Error: {}", e);
        ApiError::new(500, "Error: Can't get bastion public key".to_string())
    })?;
    Ok(HttpResponse::Ok().body(bastion_public_key))
}


// /bastion =======================================================================================

#[get("/bastions")]
pub async fn get_bastion(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let claims_admin : Result<Uuid, ApiError> = VerifyAdmin(req.clone()).await;

    match claims_admin {
        Ok(id_admin) => {
            let bastion_insere = Bastion::find_all()?;
            Ok(HttpResponse::Ok().json(bastion_insere))
        },
        Err(ApiError) => {
            let id_user :Uuid = VerifyUser(req).await?;
            let users = Ressource::ressource_user(id_user.to_string())?;
            let mut bastions: Vec<Bastion> = Vec::new();

            let mut flag=true;

            for user in users {
                
                let ressource = Ressource::find_a_ressource(user.ressource_id)?;
                let bastion = Bastion::find_un_bastion(ressource.id_bastion)?;
                for b in bastions.iter() {
                    if b.bastion_id == bastion.bastion_id {
                        flag=false;
                    }
                }

                if flag {
                    bastions.push(bastion);
                }
                else{
                    flag=true;
                }
            }

            Ok(HttpResponse::Ok().json(bastions))
        }
    }
}

#[post("/bastions")]
pub async fn create_bastion(
    bastion: web::Json<BastionModification>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let id_user:Uuid = VerifyUser(req).await?;

    //generation pair de clé pour agent et bastion
    let agent_priv = wireguard_keys::Privkey::generate();
    let agent_pub = agent_priv.pubkey();
    let bastion_priv = wireguard_keys::Privkey::generate();
    let bastion_pub = bastion_priv.pubkey();

    let liste_bastions = Bastion::find_all()?;
    //generation d'un port libre pour le bastion
    let ports: Vec<i32> = (&liste_bastions).into_iter().map(|b| b.port).collect();
    let port = generate_bastion_freeport(&ports);
    //generation d'un net_id libre pour le bastion
    let net_ids: Vec<i32> = liste_bastions.into_iter().map(|b| b.net_id).collect();
    let net_id = generate_bastion_freenetid(&net_ids);

    let mut mytoken = [0u8, 16];

    let mut alea = rand::rngs::StdRng::from_entropy();
    alea.fill_bytes(&mut mytoken);

    let engine = engine::general_purpose::STANDARD;

    let mytoken = engine.encode(mytoken);

    let bastion_id: Uuid = Uuid::new_v4();
    let bastion_id = bastion_id.to_string();

    let bastion_instance_create = BastionInstanceCreate {
        bastion_id: bastion_id.clone(),
        private_key: bastion_priv.to_base64(),
        cidr_protege: bastion.subnet_cidr.clone(),
        agent_public_key: agent_pub.to_base64(),
        agent_endpoint: bastion.agent_endpoint.clone(),
        bastion_port: port,
        net_id,
    };

    // envoyer la requete de creation de bastion a l'intancieur
    let _client = reqwest::Client::new();
    let endpoint = env::var("INSTANCIEUR_ENDPOINT");
    let endpoint = if let Ok(endpoint) = endpoint {
        endpoint
    } else {
        error!("Endpoint: {}", "Endpoint non défini");
        return Err(ApiError::new(500, "Endpoint non défini".to_string()));
    };

    let url = format!("http://{}/create/{}", endpoint, bastion_id.clone());
    let _response = _client
        .post(&url)
        .json(&bastion_instance_create)
        .send()
        .await
        .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

    //creation du bastion
    let bastion_insertion = BastionInsertable {
        bastion_id: bastion_id.clone(),
        name: bastion.name.clone(),
        subnet_cidr: bastion.subnet_cidr.clone(),
        agent_endpoint: bastion.agent_endpoint.clone(),
        pubkey: bastion_pub.to_base64(),
        port,
        net_id,
    };

    let bastion_token = BastionTokenInsertable {
        bastion_id: bastion_id.clone(),
        token: mytoken.clone(),
    };

    let bastion_insere = Bastion::create(bastion_insertion)?;
    let bastion_token = Bastion::token_create(bastion_token)?;

    let retour_api = RetourAPI {
        success: true,
        message: "Bastion créé".to_string(),
        data: ConfigAgent {
            //privkey: agent_priv.to_base64(),
            pubkey: bastion_pub.to_base64(),
            endpoint: bastion.agent_endpoint.clone(),
            target_cidr: bastion.subnet_cidr.clone(),
            token: mytoken.clone(),
        },
    };

    Ok(HttpResponse::Ok().json(retour_api))
}

// /bastion/{bastion_id} ==========================================================================

#[get("/bastions/{bastion_id}")]
pub async fn find_a_bastion(
    bastion_id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let claims_admin : Result<Uuid, ApiError> = VerifyAdmin(req.clone()).await;
    let bastion_id = bastion_id.into_inner();

    match claims_admin {
        Ok(Uuid) => {
            let bastion_affiche = Bastion::find_un_bastion(bastion_id)?;
            Ok(HttpResponse::Ok().json(bastion_affiche))
        }
        Err(ApiError) => {
            let user_id: Uuid = VerifyUser(req).await?;
            let user_id = user_id.to_string();
            let authorisation = Bastion::verification_appartenance(user_id, bastion_id.clone())
                .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
            if !authorisation {
                return Err(ApiError::new(404, "Not Found".to_string()));
            }
            let bastion = Bastion::find_un_bastion(bastion_id)?;

            Ok(HttpResponse::Ok().json(bastion))
        }
    }
}

#[patch("/bastions/{bastion_id}")]
pub async fn update_a_bastion(
    bastion_id: web::Path<String>,
    modifs: web::Json<BastionModification>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let claims_admin : Uuid = VerifyAdmin(req).await?;

    let bastion_modification = BastionModification {
        name: modifs.name.clone(),
        subnet_cidr: modifs.subnet_cidr.clone(),
        agent_endpoint: modifs.agent_endpoint.clone(),
    };

    //TODO: envoyer la requete de modification de bastion a l'intancieur

    let bastion_modif = Bastion::update_un_bastion(bastion_id.into_inner(), modifs.into_inner())?;
    Ok(HttpResponse::Ok().json(bastion_modif))
}

#[delete("/bastions/{bastion_id}")]
pub async fn delete_a_bastion(
    bastion_id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let claims_admin : Uuid = VerifyAdmin(req).await?;
    let bastion_id = bastion_id.into_inner();

    let bastion_suppression = BastionSuppression {
        id: bastion_id.clone(),
    };

    let endpoint = env::var("INSTANCIEUR_ENDPOINT");
    let endpoint = if let Ok(endpoint) = endpoint {
        endpoint
    } else {
        error!("Endpoint: {}", "Endpoint non défini");
        return Err(ApiError::new(500, "Endpoint non défini".to_string()));
    };

    // envoyer la requete de suppression de bastion a l'intancieur qui doit aussi approuver la suppression des users
    let _client = reqwest::Client::new();
    let url = format!("http://{}/delete/{}", endpoint, bastion_id);
    let _response = _client
        .post(&url)
        .send()
        .await
        .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

    let bastion_supr = suppression_bastion(bastion_id).await?;

    Ok(HttpResponse::Ok().json("supprimé"))
}

//  /bastion/{bastion_id}/users ===================================================================
/*
#[get("/bastions/{bastion_id}/users")]
pub async fn get_users(
    bastion_id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let ressources = Ressource::find_all_ressources(bastion_id.into_inner())?;
    let listeuserunique: Vec<Users> = Vec::new();
    for ressource in ressources {
        let users = Users::find_users_ressources(ressource.id)?;
        
        for u in users {
            let mut flag = true;
            for l in listeuserunique.iter() {
                if l.user_id == u.user_id {
                    flag = false;
                }
            }
            if flag {
                listeuserunique.push(u);
            }
            return Ok(HttpResponse::Ok().json(listeuserunique)); 
        }; 
    };
    Ok(HttpResponse::Ok().finish())
}

#[post("/bastions/{bastion_id}/users")]
pub async fn create_users(
    users: web::Json<UsersCreation>,
    bastion_id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let admin_id: Uuid = VerifyAdmin(req).await?;

    let bastion_id = bastion_id.into_inner();

    let liste_users = Users::find_users_ressources(ressource_id.clone())?;

    let net_ids: Vec<i32> = liste_users.into_iter().map(|b| b.net_id).collect();
    let net_id = generate_user_freenetid(&net_ids);

    let users_insertion = UsersModification {
        user_id: users.id.clone(),
        ressource_id: users.ressource_id.clone(),
        net_id: net_id.clone(),
    };

    let users = Users::create_users(users_insertion)?;

    let retour_api = RetourAPI {
        success: true,
        message: "User créé".to_string(),
        data: ConfigUser {
            id: users.user_id.clone(),
            net_id,
        },
    };

    Ok(HttpResponse::Ok().json(retour_api))
}

// /bastion/{bastion_id}/users/{user_id} =========================================================

#[get("/bastions/{bastion_id}/users/{user_id}")]
pub async fn get_a_user(
    données: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let (bastion_id, user_id) = données.into_inner();
    let users = Users::find_un_user(bastion_id, user_id)?;
    Ok(HttpResponse::Ok().json(users))
}

#[delete("/bastions/{bastion_id}/users/{user_id}")]
pub async fn delete_a_user(
    données: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let (bastion_id, user_id) = données.into_inner();
    let user_suppr = Users::delete_un_user(bastion_id, user_id)?;
    // TODO: envoyer la requete de suppression de user au bastion
    Ok(HttpResponse::Ok().json("supprimé"))
}

// /bastion/{bastion_id}/users/{user_id}/generate_wireguard" ===============================================

#[post("/bastions/{bastion_id}/users/{user_id}/generate_wireguard")]
pub async fn get_user_wireguard_status(
    req: HttpRequest,
    donnees: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiError> {
    info!("request: generation_wireguard");
    let user_id: Uuid = VerifyUser(req).await?;
    let (bastion_id, user_id) = donnees.into_inner();
    let authorisation = Bastion::verification_appartenance(user_id.clone(), bastion_id.clone())
        .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
    if !authorisation {
        return Err(ApiError::new(404, "Not Found".to_string()));
    }
    info!(
        "request: generation_wireguard, user_id: {} autorisé",
        user_id
    );
    let client_priv = wireguard_keys::Privkey::generate();
    let client_pub = client_priv.pubkey();

    let bastion_ip = env::var("BASTION_IP").expect("BASTION_IP must be set");

    let client_address = build_client_address(bastion_id.clone(), user_id.clone())?;
    let bastion_endpoint = build_endpoint_user(bastion_ip.to_string(), bastion_id.clone())?;

    let client_public_key = client_pub.to_base64();
    let client_private_key = client_priv.to_base64();

    let instance_client = InstanceClient {
        public_key: client_public_key,
        allowed_ips: client_address.clone(),
    };

    //instancier le client dans Bastion
    info!("ajout du peer dans le bastion : {:?}", instance_client);
    let _client = reqwest::Client::new();
    let url = format!("http://intern-bastion-{}:9000/adduser", bastion_id.clone());
    let _response = _client
        .post(&url)
        .json(&instance_client)
        .send()
        .await
        .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

    info!("Peer ajouté au bastion");

    update_un_user(user_id, true)?;

    let bastion_public_key = get_bastion_public_key(bastion_id.clone())?;
    let subnet_cidr = get_bastion_subnet_cidr(bastion_id)?;

    let retour_api = RetourAPI {
        success: true,
        message: "accés client créé".to_string(),
        data: ConfigClient {
            client_private_key,
            client_address,
            bastion_public_key,
            bastion_endpoint,
            subnet_cidr,
        },
    };

    Ok(HttpResponse::Ok().json(retour_api))
}
*/
// /bastion/{bastion_id}/ressources        ===================================================================

#[get("/bastions/{bastion_id}/ressources")]
pub async fn get_ressources(
    bastion_id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let claims_admin : Result<Uuid, ApiError> = VerifyAdmin(req.clone()).await;

    match claims_admin {
        Ok(id_admin) => {
            let bastion_insere = Bastion::find_all()?;
            return Ok(HttpResponse::Ok().json(bastion_insere))
        },
        Err(ApiError) => {
            let id_user :Uuid = VerifyUser(req).await?;
            let users = Ressource::ressource_user(id_user.to_string())?;
            let mut ressources: Vec<Ressource> = Vec::new();

            for user in users {
                
                let ressource = Ressource::find_a_ressource(user.ressource_id)?;
                ressources.push(ressource);
            
            
        }
        return Ok(HttpResponse::Ok().json(ressources))
    }
}

    let bastion_id = bastion_id.into_inner();
    let ressources = Ressource::find_all_ressources(bastion_id)?;
    Ok(HttpResponse::Ok().json(ressources))
}

#[delete("/bastions/{bastion_id}/ressources")]
pub async fn delete_ressources(
    bastion_id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let bastion_id = bastion_id.into_inner();
    let ressources = Ressource::find_all_ressources(bastion_id.clone())?;
    for ressource in ressources {
        let ressource_suppr = ressource_suppression(bastion_id.clone(), ressource.id).await?;
    }
    Ok(HttpResponse::Ok().json("supprimé"))
}

#[post("/bastions/{bastion_id}/ressources/create/ssh")]
pub async fn create_ssh_ressource(
    req: HttpRequest,
    donnees: web::Path<String>,
    ressource_data: web::Json<RessourceSshCreation>,
) -> Result<HttpResponse, ApiError>{
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let bastion_id = donnees.into_inner();
    let liste_ressources = Ressource::find_all_ressources(bastion_id.clone())?;

    let uuid= Uuid::new_v4();
    let uuid = uuid.to_string();

    let wid = None;
    let kid = None;
    let ressources = Ressource::find_all_ressources(bastion_id.clone())?;
    let mut sid = 0;

    for ressource in ressources{
        if ressource.id_ssh.is_some(){
            if ressource.id_ssh>Some(sid){
                sid=ressource.id_ssh.unwrap();
            }
        }
    }
    sid=sid+1;

    let ssh_insertion = SshRessourceInsertable{
        id: sid,
        id_bastion: bastion_id.clone(),
        name: ressource_data.name.clone(),
        ip_machine: ressource_data.ip_machine.clone()
    };
    let specressource = SshRessource::create_ssh_ressources(ssh_insertion)?;

    let ressource_insertion = RessourceInsertable {
        id: uuid,
        name: ressource_data.name.clone(),
        rtype: "ssh".to_string(),
        id_bastion: bastion_id,
        id_wireguard: wid,
        id_ssh: Some(sid),
        id_k8s: kid,
    };
    let ressources = Ressource::create_ressources(ressource_insertion)?;
    Ok(HttpResponse::Ok().json(ressources))
    

} 
#[post("/bastions/{bastion_id}/ressources/create/wireguard")]
pub async fn create_wireguard_ressource(
    req: HttpRequest,
    donnees: web::Path<String>,
    ressource_data: web::Json<RessourceWireguardCreation>,
) -> Result<HttpResponse, ApiError>{
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let bastion_id = donnees.into_inner();
    let liste_ressources = Ressource::find_all_ressources(bastion_id.clone())?;

    let uuid= Uuid::new_v4();
    let uuid = uuid.to_string();

    let sid = None;
    let kid = None;
    let ressources = Ressource::find_all_ressources(bastion_id.clone())?;
    let mut wid = 0;

    for ressource in ressources{
        if ressource.id_wireguard.is_some(){
            if ressource.id_wireguard>Some(wid){
                wid=ressource.id_wireguard.unwrap();
            }
        }
    }
    wid=wid+1;

    let wiregard_insertion = WireguardRessourceInsertable{
        id: wid,
        id_bastion: bastion_id.clone(),
        name: ressource_data.name.clone(),
        subnet_cidr: ressource_data.subnet_cidr.clone(),

    };
    let specressource = WireguardRessource::create_wireguard_ressources(wiregard_insertion)?;

    let ressource_insertion = RessourceInsertable {
        id: uuid,
        name: ressource_data.name.clone(),
        rtype: "wireguard".to_string(),
        id_bastion: bastion_id,
        id_wireguard: Some(wid),
        id_ssh: sid,
        id_k8s: kid,
    };
    let ressources = Ressource::create_ressources(ressource_insertion)?;
    Ok(HttpResponse::Ok().json(ressources))

}


// /bastion/{bastion_id}/ressources/{ressource_id}        ===================================================================

#[get("/bastions/{bastion_id}/ressources/{ressource_id}")]
pub async fn get_a_ressource(
    bastion_id: web::Path<String>,
    ressource_id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let admin_id: Uuid = VerifyAdmin(req).await?;

    let ressource = Ressource::find_a_ressource(ressource_id.into_inner())?;
    Ok(HttpResponse::Ok().json(ressource))
}

#[delete("/bastions/{bastion_id}/ressources/{ressource_id}")]
pub async fn delete_a_ressource(
    bastion_id: web::Path<String>,
    ressource_id: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let ressource_id = ressource_id.into_inner();
    let bastion_id = bastion_id.into_inner();
    //TODO: envoyer la requete de suppression de ressource a l'intancieur
    let ressource_suppr = ressource_suppression(bastion_id, ressource_id).await?;
    Ok(HttpResponse::Ok().json("supprimé"))
}

#[post("/bastions/{bastion_id}/ressources/{ressource_id}/getressourcecredentials/ssh")]
pub async fn generate_ssh_access_credentials(
    req: HttpRequest,
    donnees: web::Path<(String, String)>,
    sshdata: web::Json<RessourceCredentialsSsh>,
) -> Result<HttpResponse, ApiError>{

    let client = reqwest::Client::new();

    //TODO url
    let user_id: Uuid = VerifyUser(req).await?;
    let (bastion_id, ressource_id) = donnees.into_inner();
    let user_id = user_id.to_string();
    let authorisation = Bastion::verification_appartenance(user_id.clone(), bastion_id.clone())
        .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
    if !authorisation {
        return Err(ApiError::new(404, "Not Found".to_string()));
    }

    let authorisation: bool = Ressource::verification_appartenance(user_id.clone(), ressource_id.clone())
        .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
    if !authorisation {
        return Err(ApiError::new(404, "Not Found".to_string()));
    }

    let ressource = Ressource::find_a_ressource(ressource_id.clone())?;
        //TODO
    let sshcredentials = UserConfigSshInsertable {
        uuid_user: user_id.clone(),
        uuid_ressource: ressource_id.clone(),
        pubkey: sshdata.pubkey.clone(),
        username: sshdata.username.clone(),
    };

    let configrequest = ConfigSshInstanceCreate{
        uuid_user: user_id.clone(),
        uuid_ressource: ressource_id.clone(),
        pubkey: sshdata.pubkey.clone(),
        username: sshdata.username.clone(),
    };

    let _test=UserConfigSsh::userconfigsshcreate(sshcredentials);

    let url = format!("??");
    let _response = client
        .post(&url)
        .json(&configrequest)
        .send()
        .await
        .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;


    Ok(HttpResponse::Ok().finish())

}

#[post("/bastions/{bastion_id}/ressources/{ressource_id}/getressourcecredentials/wireguard")]

pub async fn generate_wireguard_access_credentials(
    req: HttpRequest,
    donnees: web::Path<(String, String)>,
    wireguarddata: web::Json<RessourceCredentialsWireguard>,
) -> Result<HttpResponse, ApiError>{

    let client = reqwest::Client::new();

    //TODO url
    let user_id: Uuid = VerifyUser(req).await?;
    let (bastion_id, ressource_id) = donnees.into_inner();
    let user_id = user_id.to_string();
    let authorisation = Bastion::verification_appartenance(user_id.clone(), bastion_id.clone())
        .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
    if !authorisation {
        return Err(ApiError::new(404, "Not Found".to_string()));
    }

    let authorisation: bool = Ressource::verification_appartenance(user_id.clone(), ressource_id.clone())
        .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
    if !authorisation {
        return Err(ApiError::new(404, "Not Found".to_string()));
    }

    let ressource = Ressource::find_a_ressource(ressource_id.clone())?;

    let wireguardcredentials = UserConfigWireguardInsertable {
        uuid_user: user_id.clone(),
        uuid_ressource: ressource_id.clone(),
        pubkey: wireguarddata.pubkey.clone(),
        user_net_id: wireguarddata.user_net_id.clone(),
    };

    let configrequest = ConfigWireguardInstanceCreate{
        uuid_user: user_id.clone(),
        uuid_ressource: ressource_id.clone(),
        pubkey: wireguarddata.pubkey.clone(),
        user_net_id: wireguarddata.user_net_id.clone(),
    };

    let _test=UserConfigWireguard::userconfigwireguardcreate(wireguardcredentials);

    let url = format!("??");
    let _response = client
        .post(&url)
        .json(&configrequest)
        .send()
        .await
        .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;


    Ok(HttpResponse::Ok().finish())

}
#[post("/bastions/{bastion_id}/ressources/{ressource_id}")]
pub async fn start_session(
    req: HttpRequest,
    donnees: web::Path<(String, String)>
) -> Result<HttpResponse, ApiError>{
    let client = reqwest::Client::new();


    let user_id: Uuid = VerifyUser(req).await?;
    let (bastion_id, ressource_id) = donnees.into_inner();
    let user_id = user_id.to_string();
    let authorisation = Bastion::verification_appartenance(user_id.clone(), bastion_id.clone())
        .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
    if !authorisation {
        return Err(ApiError::new(404, "Not Found".to_string()));
    }

    let authorisation: bool = Ressource::verification_appartenance(user_id.clone(), ressource_id.clone())
        .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
    if !authorisation {
        return Err(ApiError::new(404, "Not Found".to_string()));
    }
    let ressource = Ressource::find_a_ressource(ressource_id.clone())?;
    if ressource.rtype == "wireguard"{
            UserConfigWireguard::start_wireguard_session(user_id, ressource_id).await?;

    }
    else if ressource.rtype == "ssh"{
        UserConfigSsh::start_ssh_session(user_id, ressource_id).await?;
    }

    
    
     Ok(HttpResponse::Ok().finish())
}

#[post("/bastions/{bastion_id}/ressources/{ressource_id}")]
pub async fn stop_session(
    req: HttpRequest,
    donnees: web::Path<(String, String)>
) -> Result<HttpResponse, ApiError>{
    let client = reqwest::Client::new();

    //TODO url
    let user_id: Uuid = VerifyUser(req).await?;
    let (bastion_id, ressource_id) = donnees.into_inner();
    let user_id = user_id.to_string();
    let authorisation = Bastion::verification_appartenance(user_id.clone(), bastion_id.clone())
        .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
    if !authorisation {
        return Err(ApiError::new(404, "Not Found".to_string()));
    }

    let authorisation: bool = Ressource::verification_appartenance(user_id.clone(), ressource_id.clone())
        .map_err(|_| ApiError::new(404, "Not Found".to_string()))?;
    if !authorisation {
        return Err(ApiError::new(404, "Not Found".to_string()));
    }
    let ressource = Ressource::find_a_ressource(ressource_id.clone())?;
    if ressource.rtype == "wireguard"{
            UserConfigWireguard::stop_wireguard_session(user_id, ressource_id).await?;

    }
    else if ressource.rtype == "ssh"{
        UserConfigSsh::stop_ssh_session(user_id, ressource_id).await?;
    }

    
    
     Ok(HttpResponse::Ok().finish())
}


// /bastion/{bastion_id}/ressources/{ressource_id}/users        ===================================================================
#[get("/bastions/{bastion_id}/ressources/{ressource_id}/users")]
pub async fn get_user(
    donnees: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError>{
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let (bastion_id, ressource_id) = donnees.into_inner();
    let users = Users::find_users_ressources(ressource_id.clone())?;
    Ok(HttpResponse::Ok().json(users))

}

#[post("/bastions/{bastion_id}/ressources/{ressource_id}/users")]
pub async fn create_user(
    donnees: web::Path<(String, String)>,
    req: HttpRequest,
    user: web::Json<UsersCreation>,
) -> Result<HttpResponse, ApiError>{
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let (bastion_id, ressource_id) = donnees.into_inner();

    let liste_users = Users::find_users_ressources(ressource_id.clone())?;
    let mut net_id: i32 = 0;
    for user in liste_users{
        if user.net_id > net_id{
            net_id = user.net_id;
        }
    }
    net_id = net_id + 1;

    let users_insertion = UsersModification {
        user_id: user.id.clone(),
        ressource_id: user.ressource_id.clone(),
        net_id: net_id.clone(),
    };

    let users = Users::create_users(users_insertion)?;

    let retour_api = RetourAPI {
        success: true,
        message: "User créé".to_string(),
        data: ConfigUser {
            id: users.user_id.clone(),
            net_id,
        },
    };
    Ok(HttpResponse::Ok().json(retour_api))
}

#[delete("/bastions/{bastion_id}/ressources/{ressource_id}/users")]
pub async fn delete_users(
    donnees: web::Path<(String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError>{
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let (bastion_id, ressource_id) = donnees.into_inner();
    let users = Users::find_users_ressources(ressource_id.clone())?;
    for user in users{
        user_suppression(user.user_id, ressource_id.clone()).await?;
    }
    Ok(HttpResponse::Ok().json("supprimé"))
}
// /bastion/{bastion_id}/ressources/{ressource_id}/users/{user_id}        ===================================================================
#[get("/bastions/{bastion_id}/ressources/{ressource_id}/users/{user_id}")]
pub async fn get_a_user(
    données: web::Path<(String, String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let (bastion_id,ressource_id, user_id) = données.into_inner();
    let users = Users::find_un_user(ressource_id, user_id)?;
    Ok(HttpResponse::Ok().json(users))
}

#[delete("/bastions/{bastion_id}/ressources/{ressource_id}/users/{user_id}")]
pub async fn delete_user(
    donnees: web::Path<(String, String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError>{
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let (bastion_id, ressource_id, user_id) = donnees.into_inner();
    user_suppression(user_id, ressource_id).await?;
    Ok(HttpResponse::Ok().json("supprimé"))
}

pub fn routes_bastion(cfg: &mut web::ServiceConfig) {
    cfg.service(Config_my_agent);

    cfg.service(create_bastion);
    cfg.service(get_bastion);

    cfg.service(update_a_bastion);
    cfg.service(find_a_bastion);
    cfg.service(delete_a_bastion);

    /*cfg.service(get_users);
    cfg.service(create_users);

    cfg.service(get_a_user);
    cfg.service(delete_a_user);*/

   // cfg.service(get_user_wireguard_status);

    cfg.service(get_ressources);
    cfg.service(delete_ressources);


    cfg.service(get_a_ressource);
    cfg.service(delete_a_ressource);

    cfg.service(generate_ssh_access_credentials);
    cfg.service(generate_wireguard_access_credentials);
    cfg.service(start_session);
    cfg.service(stop_session);

    /*
    cfg.service(find_server_config);
    cfg.service(update_server_config); */
}
