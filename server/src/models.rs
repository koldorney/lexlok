use diesel::{Queryable, Selectable, Insertable};
use serde::Deserialize;


#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub groups: Vec<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
}
