use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub role: String,
    pub token: String,
}

#[derive(Serialize, Queryable, Debug)]
pub struct UserResponse {
    pub name: String,
    pub email: String,
    pub role: String,
    pub division: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct UpdateRequest {
    pub email: String,
    pub update: UpdateData,
}

#[derive(Deserialize)]
pub struct UpdateData {
    pub name: String,
    pub email: String,
    pub division: String,
}

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub email: String,
}

#[derive(Serialize, Queryable, Debug)]
pub struct UserSearch {
    pub name: String,
    pub email: String,
    pub division: i32,
}

#[derive(Deserialize)]
pub struct AddRequest {
    pub name: String,
    pub email: String,
    pub pass: String,
    pub division: Option<i32>,
}
