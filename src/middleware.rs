use std::env;

use actix_web::{
    body::{ EitherBody, MessageBody },
    dev::{ ServiceRequest, ServiceResponse },
    middleware::Next,
    Error,
    HttpMessage,
    HttpResponse,
};
use jsonwebtoken::{ decode, Algorithm, DecodingKey, Validation };
use serde_json::json;
use crate::models::api::login::Claims;

pub async fn admin_middleware<B>(
    req: ServiceRequest,
    next: Next<B>
) -> Result<ServiceResponse<EitherBody<B>>, Error>
    where B: MessageBody + 'static
{
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h.to_str().unwrap_or(""),
        None => {
            return Ok(
                req.into_response(
                    HttpResponse::Unauthorized()
                        .json(json!({ "error": "Invalid Credentials" }))
                        .map_into_right_body()
                )
            );
        }
    };

    if !auth_header.starts_with("Bearer ") {
        return Ok(
            req.into_response(
                HttpResponse::Unauthorized()
                    .json(json!({ "error": "Invalid Credentials" }))
                    .map_into_right_body()
            )
        );
    }

    let token = &auth_header[7..];
    let key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let validation = Validation::new(Algorithm::HS256);
    let decoded_token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key.as_ref()),
        &validation
    );

    if let Ok(token_data) = decoded_token {
        let claims = token_data.claims;

        if claims.role != "admin" {
            return Ok(
                req.into_response(
                    HttpResponse::Unauthorized()
                        .json(json!({ "message": "Invalid Credentials" }))
                        .map_into_right_body()
                )
            );
        }
    } else {
        return Ok(
            req.into_response(
                HttpResponse::Unauthorized()
                    .json(json!({ "error": "Invalid Credentials" }))
                    .map_into_right_body()
            )
        );
    }

    let res = next.call(req).await?;
    Ok(res.map_into_left_body())
}

pub async fn auth_middleware<B>(
    req: ServiceRequest,
    next: Next<B>
) -> Result<ServiceResponse<EitherBody<B>>, Error>
    where B: MessageBody + 'static
{
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h.to_str().unwrap_or(""),
        None => {
            return Ok(
                req.into_response(
                    HttpResponse::Unauthorized()
                        .json(json!({ "error": "Invalid Credentials" }))
                        .map_into_right_body()
                )
            );
        }
    };

    if !auth_header.starts_with("Bearer ") {
        return Ok(
            req.into_response(
                HttpResponse::Unauthorized()
                    .json(json!({ "error": "Invalid Credentials" }))
                    .map_into_right_body()
            )
        );
    }

    let token = &auth_header[7..];
    let key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let validation = Validation::new(Algorithm::HS256);
    let decoded_token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key.as_ref()),
        &validation
    );

    if let Ok(token_data) = decoded_token {
        req.extensions_mut().insert(token_data.claims);
    } else {
        return Ok(
            req.into_response(
                HttpResponse::Unauthorized()
                    .json(json!({ "error": "Invalid Credentials" }))
                    .map_into_right_body()
            )
        );
    }

    let res = next.call(req).await?;
    Ok(res.map_into_left_body())
}
