use crate::schema::admins;
use crate::tools::{api_error::ApiError, db};
use actix_web::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CodeOtp {
    //Structure envoye dans le JSON
    pub url: String,
}

//Structure gestion des admins

#[derive(Serialize, Deserialize)]
pub struct AdminReceived {
    //Structure recue dans le JSON
    pub name: String,
    pub last_name: String,
    pub mail: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdminSent{ //structure envoyee vers authentication

    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub mail: String,
    pub password : String
}
#[derive(AsChangeset, Insertable)]
#[diesel(table_name = admins)]
pub struct AdminInserable {
    //Structure inseree en BDD pour ajouter un admin
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub mail: String,
}

#[derive(Queryable, Serialize)]
pub struct Admin {
    //Structure recupere par requete BDD
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub mail: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdminChangeCred {
    //Structure envoye dans le JSON pour changer password
    pub password: String,
}

impl AdminSent {
    pub fn from_admin_received(admin: AdminReceived) -> AdminSent {
        //Creation d un admin avec un id

        let id = Uuid::new_v4();

        AdminSent {
            //Renvoie la structure qui peut etre inseree en BDD
            id,
            name: admin.name,
            last_name: admin.last_name,
            mail: admin.mail,
            password: admin.password
        }
    }
}


impl AdminInserable {
    pub fn from_admin_sent(admin: AdminSent) -> AdminInserable {
        //Creation d un admin inserable

        AdminInserable {
            //Renvoie la structure qui peut etre inseree en BDD
            id : admin.id,
            name: admin.name,
            last_name: admin.last_name,
            mail: admin.mail,
        }
    }
}

impl Admin {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        //Fct pour récuperer tous les admins de la BDD
        let mut conn = db::connection()?;

        let admins = admins::table.load::<Admin>(&mut conn)?; //On recupere la liste des noms

        Ok(admins)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        //Fct pour recuperer 1 admin en particulier de la BDD

        let mut conn = db::connection()?;

        let admin = admins::table.filter(admins::id.eq(id)).first(&mut conn)?;

        Ok(admin)
    }

    pub fn create(admin: AdminSent) -> Result<Admin, ApiError> {
        //Fct pour créer un admin à partir du JSON envoyé a l'api
        let mut conn = db::connection()?;

        let admin = AdminInserable::from_admin_sent(admin);

        let admin = diesel::insert_into(admins::table)
            .values(admin)
            .get_result(&mut conn)?;

        Ok(admin)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        //Supprimer un admin de la BDD
        let mut conn = db::connection()?;

        let res = diesel::delete(admins::table.filter(admins::id.eq(id))).execute(&mut conn)?;

        Ok(res)
    }
}

pub fn premiere_utilisation_bastion(admin: AdminSent) -> Result<Admin, ApiError> {
    //Fct pour creer l'admin par défaut

    let mut conn = db::connection()?;

    let admins = admins::table.load::<Admin>(&mut conn)?;

    if admins.is_empty() {
        //Si aucun utilisateur, on creer l'utilisateur par défaut

        let admin = Admin::create(admin)?; //Insertion de cet utilisateur dans la BDD

        Ok(admin)
    } else {
        Err(ApiError {
            status_code: 404,
            message: " ".to_string(),
        })
    }
}
