use crate::schema::users;
use crate::tools::{api_error::ApiError, db};
use actix_web::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct MailUser {
    //Structure recue dans le JSON pour trouver des utilisateurs
    pub mail: Option<String>,
}

#[derive(Serialize)]
pub struct CodeOtp {
    //Structure envoye dans le JSON
    pub url: String,
}

//Structure gestion des users

#[derive(Deserialize)]
pub struct UserReceived {
    //Structure recue dans le JSON pour ajouter un user
    pub name: String,
    pub last_name: String,
    pub mail: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserSent {
    //structure envoyee vers authentication
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub mail: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserInserable {
    //Structure inseree en BDD pour ajouter un user
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub mail: String,
}

#[derive(Queryable, Serialize)]
pub struct User {
    //Structure recupere par requete BDD
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub mail: String,
}

#[derive(Deserialize)]
pub struct UserChangeCred {
    //Structure envoye dans le JSON pour changer password
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSentAuthentication {
    //structure envoyee vers authentication
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub mail: String,
    pub claims: String,
}

impl UserSent {
    pub fn from_user_received(user: UserReceived) -> UserSent {
        //Creation d un user avec un id

        let id = Uuid::new_v4();

        UserSent {
            //Renvoie la structure qui peut etre envoyee vers ms authentication
            id,
            name: user.name,
            last_name: user.last_name,
            mail: user.mail,
            password: user.password,
        }
    }
}

impl UserInserable {
    pub fn from_user_sent(user: UserSent) -> UserInserable {
        //Creation d un user inserable

        UserInserable {
            //Renvoie la structure qui peut etre inseree en BDD
            id: user.id,
            name: user.name,
            last_name: user.last_name,
            mail: user.mail,
        }
    }
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        //Fct pour récuperer tous les users de la BDD
        let mut conn = db::connection()?;

        let users = users::table.load::<User>(&mut conn)?; //On recupere la liste des noms

        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        //Fct pour recuperer 1 user en particulier de la BDD

        let mut conn = db::connection()?;

        let user = users::table.filter(users::id.eq(id)).first(&mut conn)?;

        Ok(user)
    }

    pub fn create(user: UserSent) -> Result<User, ApiError> {
        //Fct pour créer un user à partir du JSON envoyé a l'api
        let mut conn = db::connection()?;

        let user = UserInserable::from_user_sent(user);

        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        //Supprimer un user de la BDD
        let mut conn = db::connection()?;

        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(&mut conn)?;

        Ok(res)
    }

    pub fn find_by_mail_pattern(mail: String) -> Result<Vec<Self>, ApiError> {
        //Fct pour récuperer tous les users de la BDD
        let mut conn = db::connection()?;

        let mail = format!("%{}%", mail);

        let users = users::table
            .filter(users::mail.ilike(mail))
            .load::<User>(&mut conn)?; //On recupere la liste des noms

        Ok(users)
    }

    pub fn add_user_extern(user: UserSentAuthentication) -> Result<User, ApiError> {
        let mut conn = db::connection()?;

        let user = UserInserable::from_user_sent(UserSent {
            id: user.id,
            name: user.name,
            last_name: user.last_name,
            mail: user.mail,
            password: "".to_string(),
        });

        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&mut conn)?;

        Ok(user)
    }
}
