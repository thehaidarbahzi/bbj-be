use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::schema::users;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub pass: String,
    pub role: String,
    pub division_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub pass: &'a str,
    pub role: &'a str,
    pub division_id: i32,
}
