use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use sha2::{ Sha256, Digest };

use crate::models::api::user::AddRequest;
use crate::models::api::user::UpdateData;
use crate::models::api::user::UserResponse;
use crate::models::api::user::UserSearch;
use crate::models::db::user::{ NewUser, User };
use crate::schema::{ users, divisions };
use crate::DbPool;

pub fn fetch_users(pool: web::Data<DbPool>) -> QueryResult<Vec<UserResponse>> {
    let mut conn = pool.get().unwrap();

    users::table
        .inner_join(divisions::table.on(users::division_id.eq(divisions::id)))
        .select((
            users::name,
            users::email,
            users::role,
            divisions::name,
            users::created_at,
            users::updated_at,
        ))
        .order(users::name.desc())
        .load::<UserResponse>(&mut conn)
}

pub fn add_user(pool: web::Data<DbPool>, data: AddRequest) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().unwrap();

    let mut hasher = Sha256::new();
    hasher.update(data.pass.as_bytes());
    let hashed = hex::encode(hasher.finalize());

    let new_user = NewUser {
        name: &data.name,
        email: &data.email,
        pass: &hashed,
        role: "user",
        division_id: data.division.unwrap(),
    };

    conn.transaction(|conn| {
        diesel::insert_into(users::table).values(&new_user).execute(conn)?;

        users::table.order(users::id.desc()).first::<User>(conn)
    })
}

pub fn search_user(pool: web::Data<DbPool>, user_mail: &str) -> QueryResult<Vec<UserSearch>> {
    let mut conn = pool.get().unwrap();

    users::table
        .filter(users::email.eq(user_mail))
        .select((users::name, users::email, users::division_id))
        .load::<UserSearch>(&mut conn)
}

pub fn update_user(
    pool: web::Data<DbPool>,
    user_mail: &str,
    update_data: &UpdateData
) -> QueryResult<usize> {
    let mut conn = pool.get().unwrap();

    let current_user = search_user(pool, user_mail)?;

    let mut new_name = update_data.name.clone();
    let mut new_email = update_data.email.clone();
    let mut new_division = update_data.division.clone();

    if new_name.trim().is_empty() {
        new_name = current_user[0].name.clone();
    }
    if new_email.trim().is_empty() {
        new_email = current_user[0].email.clone();
    }
    if new_division.trim().is_empty() {
        new_division = current_user[0].division.clone().to_string();
    }

    conn.transaction(|conn| {
        diesel
            ::update(users::table.filter(users::email.eq(user_mail)))
            .set((
                users::name.eq(new_name),
                users::email.eq(new_email),
                users::division_id.eq(new_division.parse::<i32>().unwrap()),
                users::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)
    })
}

pub fn delete_user(
    pool: web::Data<DbPool>,
    user_mail: &str
) -> Result<usize, diesel::result::Error> {
    let mut conn = pool.get().unwrap();

    conn.transaction(|conn| {
        diesel::delete(users::table.filter(users::email.eq(user_mail))).execute(conn)
    })
}
