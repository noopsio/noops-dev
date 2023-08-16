use super::AppState;
use crate::{errors::Error, service::auth::AuthService};
use axum::{
    extract::{Query, State, TypedHeader},
    headers::authorization::{Authorization, Bearer},
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Deserialize;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/auth/login", get(login))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct LoginQuery {
    token: String,
}

async fn login(
    Query(login_query): Query<LoginQuery>,
    State(auth): State<AuthService>,
) -> Result<impl IntoResponse, Error> {
    let github_access_token = login_query.token;
    let jwt = auth.login(github_access_token).await?;
    Ok(Json(jwt))
}

pub async fn auth_middleware<B>(
    State(auth): State<AuthService>,
    TypedHeader(header): TypedHeader<Authorization<Bearer>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, Error> {
    let user = auth.authenticate(header.token())?;
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
