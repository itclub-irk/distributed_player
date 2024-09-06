use std::sync::Arc;

use crate::config::NodeConfig;
use crate::password::verify_password_hash;
use axum::{
    body::Body,
    extract::State,
    http::{header, Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use time;

#[derive(Serialize, Debug)]
pub struct MessageResponse {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub password: String,
}

/// Authentication checking middleware
pub async fn auth_required(
    cookie_jar: CookieJar,
    State(config): State<Arc<NodeConfig>>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<MessageResponse>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.ok_or_else(|| {
        let json_error = MessageResponse {
            message: "You are not logged in, please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(config.auth.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        let json_error = MessageResponse {
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;

    // req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

/// Handles login user procedure
pub async fn login_user_handler(
    State(config): State<Arc<NodeConfig>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<MessageResponse>)> {
    let password = body.password;

    if !verify_password_hash(password, config.auth.password_hash.clone()) {
        let json_error = MessageResponse {
            message: "Invalid password".to_string(),
        };

        return Err((StatusCode::BAD_REQUEST, Json(json_error)));
    };

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims { exp, iat };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.auth.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::seconds(config.auth.jwt_maxage))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(
        json!(MessageResponse {
            message: "Authentication success".to_string()
        })
        .to_string(),
    );
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

/// Handles logout user
pub async fn logout_user_handler() -> Result<impl IntoResponse, (StatusCode, Json<MessageResponse>)>
{
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(
        json!(MessageResponse {
            message: "Logout success".to_string()
        })
        .to_string(),
    );
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}
