use std::{collections::HashMap, env};

use crate::admin::*;
use crate::api_error::ApiError;

use actix_web::{ delete, get, post,patch,  web, HttpResponse};
use actix_session::{Session};
use google_authenticator::{GoogleAuthenticator, ErrorCorrectionLevel};
use jsonwebtoken::{ encode, Header, EncodingKey};
use serde_json::json;

//Pour s'enregistrer en tant qu'admin

#[get("/admins")]
async fn find_all_admins(session: Session) -> Result<HttpResponse, ApiError> { //Recupere la liste des admins

        let _claims = verifier_session(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

        let admins = Admin::find_all()?;

        Ok(HttpResponse::Ok().json(admins)) //Retourne la liste

}


#[get("/admins/{id}")]
async fn find_admin(session: Session, id: web::Path<i32>) -> Result<HttpResponse, ApiError> { //Recupere un admin

    let _claims = verifier_session(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    let admin = Admin::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(admin))  //Retourne l'admin


}


#[post("/admins")]
async fn create_admin(session: Session, admin: web::Json<AdminMessage>) -> Result<HttpResponse, ApiError> { //Enregistre un admin


    let _claims = verifier_session(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    let admin = admin.into_inner();

    let url = env::var("AUTHENTICATION_URL").expect("AUTHENTICATION_URL must be set")+ "admins" ;

    let jwt = session.get::<String>("claim").unwrap().unwrap(); //on recupere le JWT du cookie de session

        //creation du JSON a poster vers le micro service authentification
    let mut map = HashMap::new();  
    map.insert("name".to_string(), admin.name.clone());
    map.insert("last_name".to_string(), admin.last_name.clone());
    map.insert("mail".to_string(), admin.mail.clone());
    map.insert("password".to_string(), admin.password.clone());
    map.insert("claim".to_string(), jwt);

        

    let client = reqwest::Client::new();   //Envoie une requete au micro service authentification pour ajouter l admin dans sa BDD
    let  _rest = client.post(url)
        .json(&map)
        .send()
        .await.map_err(|_| ApiError::new(400, "Erreur insertion dans authentification".to_string()))?;


    let admin = Admin::create(admin)?;  //Insertion de l'admin en bdd

        //On insere en bdd apres la reponse de micro-service authentification 
        //si l'insertion dans authentification ne marche pas, 
        //ca ne va pas etre insere dans la bdd gestion admin

    Ok(HttpResponse::Ok().json(admin))

}

#[post("/admins/{id}/otp")]
async fn ajout_2fa(session: Session,id: web::Path<i32>) -> Result<HttpResponse, ApiError> { //Fct pour ajouter la 2fa

    println!("ok0");

    
    let mut claims = verifier_session_simple(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;
//l admin a jamais active double authenf
    let id = id.into_inner();


    if claims.id == id && claims.mfa_active == false { 

        let jwt = session.get::<String>("claim").unwrap().unwrap(); //on recupere le JWT du cookie de session

        let auth = GoogleAuthenticator::new();

        let secret = auth.create_secret(32);
    

            //creation du JSON a poster vers le micro service authentification
        let mut map = HashMap::new();  
        map.insert("password".to_string(), secret.clone());
        map.insert("claim".to_string(), jwt);

        let url = env::var("AUTHENTICATION_URL").expect("AUTHENTICATION_URL must be set") + "admins/" + &id.to_string().to_owned() + "/otp" ;

        let url = url.as_str();

        let client = reqwest::Client::new();   //Envoie une requete au micro service authentification pour ajouter l admin dans sa BDD
        let  _rest = client.post(url)
            .json(&map)
            .send()
            .await.map_err(|_| ApiError::new(400, "Erreur insertion dans authentification".to_string()))?;

        let url= auth.qr_code_url(&secret, &claims.mail, "bastion_mania", 200, 200, ErrorCorrectionLevel::High);

        let code = CodeOtp{url : url};

        let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

        claims.mfa_active = true;

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap(); //Creation du jwt

        session.insert("claim", token).unwrap();

        Ok(HttpResponse::Ok().json(code))


    }

    else{
        Err(ApiError::new(403, "Interdit".to_string()))

    }

}



#[patch("/admins/{id}")]
async fn update(session: Session,id: web::Path<i32>, cred: web::Json<AdminChangeCred>) -> Result<HttpResponse, ApiError> { //Fct pour mettre a jour donnée de l'admin

    let mut claims = verifier_session_simple(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    let id = id.into_inner();

    let cred = cred.into_inner();

    if claims.id == id && claims.complete_authentication == true{ 

        let jwt = session.get::<String>("claim").unwrap().unwrap(); //on recupere le JWT du cookie de session

            //creation du JSON a poster vers le micro service authentification
        let mut map = HashMap::new();  
        map.insert("password".to_string(), cred.password.clone());
        map.insert("claim".to_string(), jwt);

        let url = env::var("AUTHENTICATION_URL").expect("AUTHENTICATION_URL must be set")+ "admins/" + &id.to_string().to_owned() ;

        let url = url.as_str();

        let client = reqwest::Client::new();   //Envoie une requete au micro service authentification pour ajouter l admin dans sa BDD
        let  _rest = client.patch(url)
            .json(&map)
            .send()
            .await.map_err(|_| ApiError::new(400, "Erreur insertion dans authentification".to_string()))?;

        let admin = Admin::update_password(id, cred)?; //Modifie le mot de passe dans la BDD

        let secret = env::var("KEY_JWT").expect("erreur chargement cle jwt");

        claims.change_password = true;

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap(); //Creation du jwt

        session.insert("claim", token).unwrap();

        Ok(HttpResponse::Ok().json(admin))


    }

    else{
        Err(ApiError::new(403, "Interdit".to_string()))

    }

}


#[delete("/admins/{id}")]
async fn delete_admin( session: Session,id: web::Path<i32>) -> Result<HttpResponse, ApiError> {  //Supprime un admin

    let _claims = verifier_session(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    let id = id.into_inner();

    let url = env::var("AUTHENTICATION_URL").expect("AUTHENTICATION_URL must be set")+ "admins/" + &id.to_string().to_owned() ;

    let jwt = session.get::<String>("claim").unwrap().unwrap(); //on recupere le JWT du cookie de session

    //creation du JSON a poster vers le micro service authentification
    let mut map = HashMap::new();  
        map.insert("claim".to_string(), jwt);

    let client = reqwest::Client::new();   //Envoie une requete au micro service authentification pour supprimer l'admin dans sa BDD
    let  _rest = client.delete(url)
        .json(&map)
        .send()
        .await.map_err(|_| ApiError::new(400, "Erreur insertion dans authentification".to_string()))?;

    let num_deleted = Admin::delete(id)?;

    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))


}

#[post("/premiere_utilisation")]
async fn premiere_utilisation(admin: web::Json<AdminMessage>) -> Result<HttpResponse, ApiError> { //Enregistre un admin

    let admin = admin.into_inner();

    let url = env::var("AUTHENTICATION_URL").expect("AUTHENTICATION_URL must be set")+ "premiere_utilisation" ;

    
        //creation du JSON a poster vers le micro service authentification
    let mut map = HashMap::new();  
    map.insert("name".to_string(), admin.name.clone());
    map.insert("last_name".to_string(), admin.last_name.clone());
    map.insert("mail".to_string(), admin.mail.clone());
    map.insert("password".to_string(), admin.password.clone());
    map.insert("claim".to_string(), " ".to_string());

        

    let client = reqwest::Client::new();   //Envoie une requete au micro service authentification pour ajouter l admin dans sa BDD
    let  _rest = client.post(url)
        .json(&map)
        .send()
        .await.map_err(|_| ApiError::new(404, "Not Found".to_string()))?;


    let admin = premiere_utilisation_bastion(admin)?;  //Insertion de l'admin en bdd

        //On insere en bdd apres la reponse de micro-service authentification 
        //si l'insertion dans authentification ne marche pas, 
        //ca ne va pas etre insere dans la bdd gestion admin

    Ok(HttpResponse::Ok().json(admin))

}

#[get("/logout")]
async fn logout(session: Session) -> Result<HttpResponse, ApiError> {

    let _claims = verifier_session(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    session.purge();
        
    Ok(HttpResponse::Ok().finish())

}





pub fn routes_admin_utilisation(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all_admins);
    cfg.service(find_admin);
    cfg.service(create_admin);
    cfg.service(ajout_2fa);
    cfg.service(update);
    cfg.service(delete_admin);
    cfg.service(premiere_utilisation);
    cfg.service(logout);
}