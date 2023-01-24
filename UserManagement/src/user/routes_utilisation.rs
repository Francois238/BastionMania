use std::{collections::HashMap, env};

use crate::user::*;
use crate::api_error::ApiError;

use actix_web::{ delete, get, post,patch,  web, HttpResponse};
use actix_session::{Session};
use google_authenticator::{GoogleAuthenticator, ErrorCorrectionLevel};
use serde_json::json;
use jsonwebtoken::{ encode, Header, EncodingKey};

//Pour s'enregistrer en tant qu'admin

#[get("/users")]
async fn find_all_users(session: Session) -> Result<HttpResponse, ApiError> { //Recupere la liste des users

        let _claims = verifier_session_admin(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

        let users = User::find_all()?;

        Ok(HttpResponse::Ok().json(users)) //Retourne la liste


}


#[get("/users/{id}")]
async fn find_user(session: Session, id: web::Path<i32>) -> Result<HttpResponse, ApiError> { //Recupere un user

    let _claims = verifier_session_admin(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;


    let user = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))  //Retourne l'user



}

#[get("/me")]
async fn find_user_me(session: Session) -> Result<HttpResponse, ApiError> { //Recupere un user

    let claims = verifier_session_simple(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;


    let user = User::find(claims.id)?;
    Ok(HttpResponse::Ok().json(user))  //Retourne l'user



}


#[post("/users")]
async fn create_user(session: Session, user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> { //Enregistre un user


    let _claims = verifier_session_admin(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    let user = user.into_inner();

    let jwt = session.get::<String>("claim").unwrap().unwrap(); //on recupere le JWT du cookie de session

    let url = env::var("AUTHENTICATION_URL").expect("AUTHENTICATION_URL must be set") + "users";

        //creation du JSON a poster vers le micro service authentification
    let mut map = HashMap::new();  
        map.insert("name".to_string(), user.name.clone());
        map.insert("last_name".to_string(), user.last_name.clone());
        map.insert("mail".to_string(), user.mail.clone());
        map.insert("password".to_string(), user.password.clone());
        map.insert("claim".to_string(), jwt);

        

    let client = reqwest::Client::new();   //Envoie une requete au micro service authentification pour ajouter l user dans sa BDD
    let  _rest = client.post(url)
        .json(&map)
        .send()
        .await.map_err(|_| ApiError::new(404, "Not Found".to_string()))?;


    let user = User::create(user)?;  //Insertion de l'user en bdd

    //On insere en bdd apres la reponse de micro-service authentification 
    //si l'insertion dans authentification ne marche pas, 
    //ca ne va pas etre insere dans la bdd gestion user

    Ok(HttpResponse::Ok().json(user))

}

#[patch("/users/{id}")]
async fn update(session: Session,id: web::Path<i32>, cred: web::Json<UserChangeCred>) -> Result<HttpResponse, ApiError> { //Fct pour mettre a jour donnée de l'admin

    let mut claims = verifier_session_simple(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    let id = id.into_inner();

    let cred = cred.into_inner();


    if claims.id == id && claims.complete_authentication == true{ //le user est completement authentifie

        let jwt = session.get::<String>("claim").unwrap().unwrap(); //on recupere le JWT du cookie de session

            //creation du JSON a poster vers le micro service authentification
        let mut map = HashMap::new();  
        map.insert("password".to_string(), cred.password.clone());
        map.insert("claim".to_string(), jwt);

        let url = env::var("AUTHENTICATION_URL").expect("AUTHENTICATION_URL must be set") + "users/" + &id.to_string().to_owned() ;

        let url = url.as_str();

        let client = reqwest::Client::new();   //Envoie une requete au micro service authentification pour modifier user password dans sa BDD
        let  _rest = client.patch(url)
            .json(&map)
            .send()
            .await.map_err(|_| ApiError::new(400, "Erreur insertion dans authentification".to_string()))?;

        let admin = User::update_password(id, cred)?; //Modifie le mot de passe dans la BDD

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

#[post("/users/{id}/otp")]
async fn ajout_2fa(session: Session,id: web::Path<i32>) -> Result<HttpResponse, ApiError> { //Fct pour activer la 2fa

    let mut claims = verifier_session_simple(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    //le user a jamais active double authenf

    let id = id.into_inner();


    if claims.id == id && claims.mfa_active == false { 

        let jwt = session.get::<String>("claim").unwrap().unwrap(); //on recupere le JWT du cookie de session

        let auth = GoogleAuthenticator::new();

        let secret = auth.create_secret(32);
    

            //creation du JSON a poster vers le micro service authentification
        let mut map = HashMap::new();  
        map.insert("password".to_string(), secret.clone());
        map.insert("claim".to_string(), jwt);

        let url = env::var("AUTHENTICATION_URL").expect("AUTHENTICATION_URL must be set") + "users/" + &id.to_string().to_owned() + "/otp" ;

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


#[delete("/users/{id}")]
async fn delete_user( session: Session,id: web::Path<i32>) -> Result<HttpResponse, ApiError> {  //Supprime un user

    let _claims = verifier_session_admin(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    let id = id.into_inner();

    let url = env::var("AUTHENTICATION_URL").expect("AUTHENTICATION_URL must be set") + "users/"+ &id.to_string().to_owned() ;

    let jwt = session.get::<String>("claim").unwrap().unwrap(); //on recupere le JWT du cookie de session

    //creation du JSON a poster vers le micro service authentification
    let mut map = HashMap::new();  
        map.insert("claim".to_string(), jwt);

    let client = reqwest::Client::new();   //Envoie une requete au micro service authentification pour supprimer le user dans sa BDD
    let  _rest = client.delete(url)
        .json(&map)
        .send()
        .await.map_err(|_| ApiError::new(400, "Erreur insertion dans authentification".to_string()))?;

    let num_deleted = User::delete(id)?;

    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))


}

#[get("/logout")]
async fn logout(session: Session) -> Result<HttpResponse, ApiError> {

    let _claims = verifier_session(&session).ok_or(ApiError::new(404, "Not Found".to_string())).map_err(|e| e)?;

    session.purge();
        
    Ok(HttpResponse::Ok().finish())

    

    
}





pub fn routes_user_utilisation(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all_users);
    cfg.service(find_user);
    cfg.service(find_user_me);
    cfg.service(create_user);
    cfg.service(ajout_2fa);
    cfg.service(update);
    cfg.service(delete_user);
    cfg.service(logout);
}
