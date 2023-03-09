use diesel::{Queryable, AsChangeset, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::session;

#[derive(Queryable, Serialize)]
pub struct Session{
    pub id: string,
    pub id_ressource: string,
    pub id_user: string,
    pub temps_fin: i32
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name="session"]
pub struct SessionInsertable{
    pub id_ressource: string,
    pub id_user: string,
    pub temps_fin: i32
}