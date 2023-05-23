use actix_web::{
    delete,
    get,
    patch, post, web, HttpResponse, HttpRequest,
};
use base64::{engine, Engine};
use rand::{SeedableRng, RngCore};
use std::env;

use crate::{api::*, model::{claims::{VerifyAdmin, VerifyUser}, agentproof::AgentProof, ressourcecredentialsssh::{RessourceCredentialsSsh, ConfigSshInstanceCreate, ActivationSshSession}, ressourcecredentialwireguard::{ActivationWireguardSession, ConfigWireguardInstanceCreate}}, entities::{userconfigssh::UserConfigSshInsertable, userconfigwireguard::UserConfigWireguardInsertable}};
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

#[post("/agent")]
pub async fn Config_my_agent(
    token: web::Json<AgentProof>,
    config_agent: web::Json<ConfigAgent>,
) -> Result<HttpResponse, ApiError> {
    let my_bastion=Bastion::token_find(token.into_inner().token)?;
    let client = reqwest::Client::new();

    let url = format!("http://bastion-internal-{}:9000/agent", my_bastion.bastion_id);
    let _response = client
        .post(&url)
        .json(&config_agent.into_inner())
        .send()
        .await
        .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

    let _suppr = Bastion::token_delete(my_bastion)?;

    Ok(HttpResponse::Ok().finish())
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

    let _users = Users::delete_all_users(bastion_id.clone())?;

    let bastion_suppr = Bastion::delete_un_bastion(bastion_id)?;
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
#[post("/bastions/{bastion_id}/ressources")]
pub async fn create_ressources(
    bastion_id: web::Path<String>,
    nom: web::Json<RessourceCreation>,
    rtype: web::Json<RessourceCreation>,
    pers_subnet_cidr: web::Json<WireguardRessourceCreation>,
    ip_machine: web::Json<SshRessourceCreation>,
    ip_cluster: web::Json<K8sRessourceCreation>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let admin_id: Uuid = VerifyAdmin(req).await?;
    let bastion_id = bastion_id.into_inner();
    let liste_ressources = Ressource::find_all_ressources(bastion_id.clone())?;

    let uuid= Uuid::new_v4();
    let uuid = uuid.to_string();

    let name = nom.into_inner().name;
    let rtype = rtype.into_inner().rtype;

    if rtype=="wireguard"{
        //let wids: Vec<i32> = (&liste_ressources).into_iter().filter(|b| b.rtype=="wireguard").map(|b| b.id_wireguard).collect();
        //let wid = generate_ressource_wireguard_freenetid(&wids);
        let sid = None;
        let kid = None;

        let wiregard_insertion = WireguardRessourceInsertable{
            id_bastion: bastion_id.clone(),
            name: name.clone(),
            subnet_cidr: pers_subnet_cidr.into_inner().subnet_cidr,

        };
        let specressource = WireguardRessource::create_wireguard_ressources(wiregard_insertion)?;

        let ressource_insertion = RessourceInsertable {
            id: uuid,
            name,
            rtype,
            id_bastion: bastion_id,
            id_wireguard: Some(specressource.id),
            id_ssh: sid,
            id_k8s: kid,
        };
        let ressources = Ressource::create_ressources(ressource_insertion)?;
        Ok(HttpResponse::Ok().json(ressources))
    }
    else if rtype=="ssh" {
        //let sids: Vec<i32> = (&liste_ressources).into_iter().filter(|b| b.rtype=="ssh").map(|b| b.id_ssh).collect();
        //let sid = generate_ressource_ssh_freenetid(&sids);
        let wid = None;
        let kid = None;

        let ssh_insertion = SshRessourceInsertable{
            id_bastion: bastion_id.clone(),
            name: name.clone(),
            ip_machine: ip_machine.into_inner().ip_machine
        };
        let specressource = SshRessource::create_ssh_ressources(ssh_insertion)?;

        let ressource_insertion = RessourceInsertable {
            id: uuid,
            name,
            rtype,
            id_bastion: bastion_id,
            id_wireguard: wid,
            id_ssh: Some(specressource.id),
            id_k8s: kid,
        };
        let ressources = Ressource::create_ressources(ressource_insertion)?;
        Ok(HttpResponse::Ok().json(ressources))
    }
    else{
        //let kids: Vec<i32> = (&liste_ressources).into_iter().filter(|b| b.rtype=="kubernetes").map(|b| b.id_k8s).collect();
        //let kid = generate_ressource_k8s_freenetid(&kids);
        let wid = None;
        let sid = None;

        let k8s_insertion = K8sRessourceInsertable{
            id_bastion: bastion_id.clone(),
            name: name.clone(),
            ip_cluster: ip_cluster.into_inner().ip_cluster,
        };
        let specressource = K8sRessource::create_k8s_ressources(k8s_insertion)?;


        let ressource_insertion = RessourceInsertable {
            id: uuid,
            name,
            rtype,
            id_bastion: bastion_id,
            id_wireguard: wid,
            id_ssh: sid,
            id_k8s: Some(specressource.id),
        };
        let ressources = Ressource::create_ressources(ressource_insertion)?;
        Ok(HttpResponse::Ok().json(ressources))
    }

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

    let ressource = Ressource::find_a_ressource(ressource_id.clone())?;
    let users: Vec<Users> = Users::find_users_ressources(ressource.id.clone())?;

    // stop session de tout utilisateur sur la ressource
    for user in users{
        let client = reqwest::Client::new();
        if ressource.rtype == "wireguard"{
            let configs = userconfigwireguardfind(user.user_id, ressource_id.clone());

            for config in configs{
                let configrequest = ActivationWireguardSession{
                    pubkey: config.pubkey.clone(),
                    user_net_id: config.user_net_id.clone(),
                };

                let url = format!("http://api/bastions/{bastion_id}/ressources/{ressource_id}");
                let _response = client
                    .post(&url)
                    .json(&configrequest)
                    .send()
                    .await
                    .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;
            }

        } 

        else if ressource.rtype == "ssh"{
            let configs = userconfigsshfind(user.user_id, ressource_id.clone());

            for config in configs{
                let configrequest = ActivationSshSession{
                    pubkey: config.pubkey.clone(),
                    username: config.username.clone(),
                };

                let url = format!("http://api/bastions/{bastion_id}/ressources/{ressource_id}");
                let _response = client
                    .post(&url)
                    .json(&configrequest)
                    .send()
                    .await
                    .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;
            }
        }
        else{
            return Err(ApiError::new(404, "Not Found".to_string()));
        }
    };

    // supprimer les config user sur la ressource

    if ressource.rtype == "wireguard"{
        let _ = userconfigwireguarddeleteall(ressource_id.clone());
    }
    else if ressource.rtype == "ssh"{
        let _ = userconfigsshdeleteall(ressource_id.clone());
    }
    else{
        return Err(ApiError::new(404, "Not Found".to_string()));
    }

    // supprimer la ressource de la base de donnée
    let rtype = ressource.rtype;

    if rtype == "wireguard"{
        let wid = ressource.id_wireguard.ok_or(ApiError::new(404, "Not Found".to_string()))?.clone();
        let _ = WireguardRessource::delete_a_wireguard_ressource(wid, bastion_id.clone())?;
    }
    else if rtype == "ssh"{
        let sid = ressource.id_ssh.ok_or(ApiError::new(404, "Not Found".to_string()))?.clone();
        let _ = SshRessource::delete_a_ssh_ressource(sid, bastion_id.clone())?;
    }
    else{
        let kid = ressource.id_k8s.ok_or(ApiError::new(404, "Not Found".to_string()))?.clone();
        let _ = K8sRessource::delete_a_k8s_ressource(kid, bastion_id.clone())?;
    }
    let ressource = Ressource::delete_a_ressource(ressource_id,bastion_id)?;
    Ok(HttpResponse::Ok().json(ressource))
}

#[post("/bastions/{bastion_id}/ressources/{ressource_id}")]
pub async fn generate_access_credentials(
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
    let rtype = ressource.rtype;


    if rtype == "wireguard"{

        let liste_users = Users::find_users_ressources(ressource_id.clone())?;

        let net_ids: Vec<i32> = liste_users.into_iter().map(|b| b.net_id).collect();
        let net_id = generate_user_freenetid(&net_ids);
        
        //instancier le client dans Bastion
        
        let _client = reqwest::Client::new();

        let wireguardconfig = UserConfigWireguardInsertable{
            uuid_user: user_id.clone(),
            uuid_ressource: ressource_id.clone(),
            pubkey: sshdata.pubkey.clone(),
            user_net_id: net_id.clone(),
        };

        let configrequest = ConfigWireguardInstanceCreate{
            uuid_user: user_id.clone(),
            uuid_ressource: ressource_id.clone(),
            pubkey: sshdata.pubkey.clone(),
            user_net_id: net_id.clone(),
        };

        let _test=userconfigwireguardcreate(wireguardconfig);
        

        let url = format!("http://intern-bastion-{}:9000/adduser", bastion_id);
        let _response = _client
            .post(&url)
            .json(&configrequest)
            .send()
            .await
            .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

       

        let retour_api = RetourAPI {
            success: true,
            message: "accés client créé".to_string(),
            data: "ok".to_string(),
        };

        return Ok(HttpResponse::Ok().json(retour_api))
    }
    else if rtype == "ssh" {
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

        let _test=userconfigsshcreate(sshcredentials);

        let url = format!("??");
        let _response = client
            .post(&url)
            .json(&configrequest)
            .send()
            .await
            .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;
    
        
        return Ok(HttpResponse::Ok().finish())

    }
    else {
        //TODO
        return Ok(HttpResponse::Ok().finish())

    }
    Ok(HttpResponse::Ok().finish())
}

#[post("/bastions/{bastion_id}/ressources/{ressource_id}")]
pub async fn start_session(
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
            let userconfig = userconfigwireguardfind(user_id.clone(), ressource_id.clone())?;

            let session = ActivationWireguardSession{
                pubkey: userconfig.pubkey.clone(),
                user_net_id: userconfig.user_net_id.clone(),
            };

            let url = format!("??");

            let _response = client
                .post(&url)
                .json(&session)
                .send()
                .await
                .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

            return Ok(HttpResponse::Ok().finish())

    }
    else if ressource.rtype == "ssh"{
        let userconfig = userconfigsshfind(user_id.clone(), ressource_id.clone())?;

        let session = ActivationSshSession{
            pubkey: userconfig.pubkey.clone(),
            username: userconfig.username.clone(),
        };

        let url = format!("??");

        let _response = client
            .post(&url)
            .json(&session)
            .send()
            .await
            .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

        return Ok(HttpResponse::Ok().finish())
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
            let userconfig = userconfigwireguardfind(user_id.clone(), ressource_id.clone())?;

            let session = ActivationWireguardSession{
                pubkey: userconfig.pubkey.clone(),
                user_net_id: userconfig.user_net_id.clone(),
            };

            let url = format!("??");

            let _response = client
                .post(&url)
                .json(&session)
                .send()
                .await
                .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

            return Ok(HttpResponse::Ok().finish())

    }
    else if ressource.rtype == "ssh"{
        let userconfig = userconfigsshfind(user_id.clone(), ressource_id.clone())?;

        let session = ActivationSshSession{
            pubkey: userconfig.pubkey.clone(),
            username: userconfig.username.clone(),
        };

        let url = format!("??");

        let _response = client
            .post(&url)
            .json(&session)
            .send()
            .await
            .map_err(|e| ApiError::new(500, format!("Error: {}", e)))?;

        return Ok(HttpResponse::Ok().finish())
    }

    
    
     Ok(HttpResponse::Ok().finish())
}


// /bastion/{bastion_id}/ressources/{ressource_id}/users        ===================================================================


// /bastion/{bastion_id}/ressources/{ressource_id}/users/{user_id}        ===================================================================

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
    cfg.service(create_ressources);

    cfg.service(get_a_ressource);
    cfg.service(delete_a_ressource);

    cfg.service(generate_access_credentials);
    cfg.service(start_session);
    cfg.service(stop_session);

    /*
    cfg.service(find_server_config);
    cfg.service(update_server_config); */
}
