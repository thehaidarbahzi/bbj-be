pub mod endpoints;
pub mod models;
pub mod routes;
pub mod schema;
pub mod controllers;
pub mod middleware;

use actix_web::{ middleware::Logger, web::{ self }, App, HttpResponse, HttpServer };
use diesel::{ r2d2::{ ConnectionManager, Pool }, mysql::MysqlConnection };
use std::env;

use crate::{ endpoints::default::not_found, routes::v1::config };

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    println!("Server launched at port 8080");

    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = establish_pool(&database_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
            .wrap(Logger::default())
            .service(web::scope("/v1").configure(config))
            .default_service(web::route().to(not_found))
    })
        .bind(("0.0.0.0", 8080))?
        .run().await
}

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

fn json_error_handler(
    err: actix_web::error::JsonPayloadError,
    _req: &actix_web::HttpRequest
) -> actix_web::Error {
    let error_message = format!("Invalid request body");
    actix_web::error::InternalError
        ::from_response(
            err,
            HttpResponse::BadRequest().json(serde_json::json!({ "message": error_message }))
        )
        .into()
}
