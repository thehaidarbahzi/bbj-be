use diesel::prelude::Queryable;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug)]
pub struct AuthUser {
    pub email: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pass: String,
}

#[derive(Queryable)]
pub struct LoginModel {
    pub email: String,
    pub role: String,
    pub pass: String,
}
