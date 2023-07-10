use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use dtos::ErrorDTO;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Token error: {}", .0)]
    TokenError(#[from] jsonwebtoken::errors::Error),
    #[error("Unknown error: {}", .0)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("{}", self);

        let (status, error_message) = match self {
            Error::TokenError(err) => match err.into_kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    (StatusCode::UNAUTHORIZED, "Invalid Token".to_string())
                }
                jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                    (StatusCode::UNAUTHORIZED, "Invalid Signature".to_string())
                }

                jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
                    (StatusCode::UNAUTHORIZED, "Invalid Issuer".to_string())
                }
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    (StatusCode::UNAUTHORIZED, "Token Expired".to_string())
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                ),
            },
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };
        (status, Json(ErrorDTO::new(&error_message))).into_response()
    }
}
