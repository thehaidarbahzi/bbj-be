use actix_web::web;
use diesel::prelude::*;
use sha2::{ Sha256, Digest };

use crate::models::api::login::LoginModel;
use crate::schema::users::{ email, pass, role, self };
use crate::DbPool;

pub fn login_user(pool: web::Data<DbPool>, user_email: &str) -> QueryResult<LoginModel> {
    let mut conn = pool.get().unwrap();

    users::table
        .filter(users::email.eq(user_email))
        .select((email, role, pass))
        .first::<LoginModel>(&mut conn)
}

pub fn user_pass(input_password: &str, stored_hash: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(input_password.as_bytes());
    let result = hasher.finalize();
    let input_hash = hex::encode(result);
    input_hash == stored_hash
}

//ganti ke return email dan pass terus di cek di handler api
