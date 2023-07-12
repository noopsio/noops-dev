use std::sync::Arc;

use crate::{database::Database, errors::Error, github, jwt::Jwt};

use axum::{
    extract::{Query, State, TypedHeader},
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

pub fn create_routes(database: Arc<Database>) -> Router {
    Router::new()
        .route("/api/auth/login", get(login))
        .with_state(database)
}

#[derive(Deserialize)]
pub struct LoginQuery {
    token: String,
}

async fn login(
    Query(login_query): Query<LoginQuery>,
    State(database): State<Arc<Database>>,
) -> Result<Response, Error> {
    let github_access_token = login_query.token;
    let gh_user = github::get_user(github_access_token.clone()).await?;
    let result = database.get_user_by_gh_id(gh_user.id)?;

    let user = match result {
        Some(user) => user,
        None => database.create_user(gh_user.id, &gh_user.email, &github_access_token)?,
    };

    let jwt = create_token(user.id)?;
    Ok((StatusCode::OK, Json(GetJWTDTO { jwt })).into_response())
}

fn create_token(subject: i32) -> anyhow::Result<String> {
    let issued_at = Jwt::create_issued_at();
    let jwt =
        Jwt::new(JWT_ISSUER, subject, issued_at, JWT_EXPIRATION_DELTA).encode(&ENCODING_KEY)?;

    Ok(jwt)
}

pub async fn auth_middleware<B>(
    State(database): State<Arc<Database>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, Error> {
    let (_, claims) = Jwt::decode(auth.token(), JWT_ISSUER, &DECODING_KEY)?;
    let user = database.get_user_by_id(claims.sub)?;
    if user.is_none() {
        return Ok((StatusCode::NOT_FOUND, "User not found").into_response());
    }
    let user = user.unwrap();
    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
