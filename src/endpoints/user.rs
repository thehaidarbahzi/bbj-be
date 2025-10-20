use std::{ env, str::FromStr };
use actix_web::{ get, delete, post, put, web, HttpResponse, Responder };
use chrono::Utc;
use email_address::EmailAddress;
use jsonwebtoken::{ encode, EncodingKey, Header };
use serde_json::json;

use crate::{
    controllers::{
        login::{ login_user, user_pass },
        user::{ add_user, delete_user, fetch_users, update_user },
    },
    models::api::{
        login::{ Claims, LoginRequest },
        user::{ AddRequest, DeleteRequest, UpdateRequest },
    },
    DbPool,
};

#[post("/login")]
async fn login(pool: web::Data<DbPool>, data: web::Json<LoginRequest>) -> impl Responder {
    if data.email.trim().is_empty() || data.pass.trim().is_empty() {
        return HttpResponse::UnprocessableEntity().json(json!({"message": "Invalid request body"}));
    }

    if let Err(_) = EmailAddress::from_str(&data.email) {
        return HttpResponse::UnprocessableEntity().json(json!({"message": "Invalid request body"}));
    }

    let user = match login_user(pool.clone(), data.email.as_str()) {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Unauthorized().json(json!({"message": "Invalid credentials"}));
        }
    };

    if !user_pass(&data.pass, &user.pass) {
        return HttpResponse::Unauthorized().json(json!({"message": "Invalid credentials"}));
    }

    let key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: user.email.clone(),
        role: user.role.clone(),
        exp: (Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(key.as_ref())) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::InternalServerError().json(
                json!({"message": "Something went wrong"})
            );
        }
    };

    HttpResponse::Ok().json(json!({ "message": "Login success", "data": [{"token": token}] }))
}

#[get("/users")]
async fn get_endpoint(pool: web::Data<DbPool>) -> impl Responder {
    match fetch_users(pool) {
        Ok(users) => HttpResponse::Ok().json(json!({"message": "success", "data": users})),
        Err(_) =>
            HttpResponse::InternalServerError().json(json!({"message": "Something went wrong"})),
    }
}

#[post("/users")]
async fn post_endpoint(pool: web::Data<DbPool>, data: web::Json<AddRequest>) -> impl Responder {
    if
        data.name.trim().is_empty() ||
        data.email.trim().is_empty() ||
        data.pass.trim().is_empty() ||
        data.division.is_none() ||
        data.division.unwrap() > 5 ||
        data.division.unwrap() < 1
    {
        return HttpResponse::UnprocessableEntity().json(json!({"message": "Invalid request body"}));
    }

    match add_user(pool, data.into_inner()) {
        Ok(_users) => HttpResponse::Ok().json(json!({"message": "Success add user"})),
        Err(e) =>
            HttpResponse::InternalServerError().json(
                json!({"message": format!("Something went wrong {}", e)})
            ),
    }
}

#[put("/users")]
async fn update_endpoint(
    pool: web::Data<DbPool>,
    data: web::Json<UpdateRequest>
) -> impl Responder {
    if
        data.email.trim().is_empty() ||
        (data.update.name.trim().is_empty() &&
            data.update.email.trim().is_empty() &&
            data.update.division.trim().is_empty())
    {
        return HttpResponse::UnprocessableEntity().json(json!({"message": "Invalid request body"}));
    }

    match update_user(pool, &data.email, &data.update) {
        Ok(_users) => HttpResponse::Ok().json(json!({"message": "Success update user"})),
        Err(_) =>
            HttpResponse::InternalServerError().json(json!({"message": "Something went wrong"})),
    }
}

#[delete("/users")]
async fn delete_endpoint(
    pool: web::Data<DbPool>,
    data: web::Json<DeleteRequest>
) -> impl Responder {
    match delete_user(pool, &data.email) {
        Ok(_user) => HttpResponse::Ok().json(json!({"message": "Success delete user"})),
        Err(_) =>
            HttpResponse::InternalServerError().json(json!({"message": "Something went wrong"})),
    }
}
