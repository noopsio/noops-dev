use crate::{errors::Error, github, jwt::Jwt};

use axum::{
    extract::{Query, TypedHeader},
    headers::authorization::{Authorization, Bearer},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use dtos::GetJWTDTO;
use jsonwebtoken::{DecodingKey, EncodingKey};
use lazy_static::lazy_static;
use serde::Deserialize;

const JWT_SECRET: &str = "ieb9upai2pooYoo9guthohchio5xie6Poo1ooThaetubahCheemaixaeZei1rah0";
const JWT_ISSUER: &str = "noops.io";
const JWT_EXPIRATION_DELTA: u64 = 3600; // 1 hour

lazy_static! {
    pub static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(JWT_SECRET.as_bytes());
    pub static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(JWT_SECRET.as_bytes());
}

pub fn create_routes() -> Router {
    Router::new().route("/api/auth/login", get(login))
}

#[derive(Deserialize)]
pub struct LoginQuery {
    token: String,
}

async fn login(Query(login_query): Query<LoginQuery>) -> Result<Response, Error> {
    let author = github::get_user(&login_query.token).await?;

    let subject = author.id.to_string();
    let issued_at = Jwt::create_issued_at();
    let gh_access_token = &login_query.token;

    let jwt = Jwt::new(
        JWT_ISSUER,
        &subject,
        issued_at,
        JWT_EXPIRATION_DELTA,
        gh_access_token,
    )
    .encode(&ENCODING_KEY)?;

    Ok((StatusCode::OK, Json(GetJWTDTO { jwt })).into_response())
}

pub async fn auth_middleware<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, Error> {
    let _ = Jwt::decode(auth.token(), JWT_ISSUER, &DECODING_KEY)?;
    // TODO Check here if github access token is still valid
    // Get user information from Database. if not Available get it from Github
    //request.headers_mut().remove(AUTHORIZATION);
    Ok(next.run(request).await)
}
