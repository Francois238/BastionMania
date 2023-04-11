use std::env;
use argon2::Config;
use rand::Rng;
use crate::tools::api_error::ApiError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce // Or `Aes128Gcm`
};

pub fn hash_password(password : String) -> Result<String, ApiError> { //Fct pour hash a String

    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    let password = argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;

    Ok(password)
}

pub fn encrypt_password(password : String) -> Result<Vec<u8>, ApiError>{ //Fct pour chiffrer le password hashe


    let secret = env::var("KEY_BDD").map_err(|_| ApiError::new(500, format!("Failed to load key")))?;

    let nonce = env::var("NONCE").map_err(|_| ApiError::new(500, format!("Failed to load nonce")))?;

    let cipher = Aes256Gcm::new_from_slice(secret.as_bytes()).map_err(|_| ApiError::new(500, format!("Failed to load nonce")))?;

    let nonce = Nonce::from_slice(nonce.as_bytes()); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(nonce, password.as_bytes().as_ref()).map_err(|_| ApiError::new(500, format!("Internal error")))?; //chiffre le mot de passe qui est hashe

    Ok(ciphertext)


}


pub fn verify_password(password: &[u8], password_verify: &[u8]) -> Result<bool, ApiError> { //Verifier mot de passe de l'user qui veut se connecter

    //On va dechiffrer le mot de passe de la BDD
    //On va comparer les hash entre le mot de passe BDD et celui envoyé a l'api

    let secret = env::var("KEY_BDD").map_err(|_| ApiError::new(500, format!("Failed to load key")))?; //Charge la cle de chiffrement

    let nonce = env::var("NONCE").map_err(|_| ApiError::new(500, format!("Failed to load nonce")))?; //Charge le nonce

    let cipher = Aes256Gcm::new_from_slice(secret.as_bytes()).map_err(|_| ApiError::new(500, format!("Failed to load nonce")))?;

    let nonce = Nonce::from_slice(nonce.as_bytes()); // 96-bits; unique per message

    let password_bdd = cipher.decrypt(nonce, password).map_err(|_| ApiError::new(500, format!("Internal error")))?;  //Dechiffre le hash du mot de passe

    let password_bdd = String::from_utf8(password_bdd).expect("Echec lecture"); //Transforme le mot de passe hashe en String pour comparer

    argon2::verify_encoded(password_bdd.as_str(), password_verify)   //Comparaison des hashs
        .map_err(|_| ApiError::new(403, format!("Failed to verify password")))
}
