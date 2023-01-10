use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable)]
#[derive(Serialize)]
struct Bastion{
    pub id: String,
    pub name: String,
    pub protocols: String
}