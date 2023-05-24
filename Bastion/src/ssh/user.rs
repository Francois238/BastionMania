use serde::{Deserialize, Serialize};

use crate::ssh::model::PublicKey;

/// Représente un utilisateur d'une ressource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHUser {
    /// L'id utilisateur dans BastionMania
    pub id: String,
    /// Le nom d'utilisateur sur la ressource
    pub name: String,
    /// La clé publique de l'utilisateur pour se connecter au bastion
    pub public_key: PublicKey,
}
