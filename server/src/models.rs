use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};
use crate::schema::users;
#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub groups: Option<Vec<Option<i32>>>,
}

#[derive(Insertable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
