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
    Token(#[from] jsonwebtoken::errors::Error),
    #[error("Unknown error: {}", .0)]
    Unknown(#[from] anyhow::Error),

    #[error("User not registered")]
    UserNotRegistered,

    #[error("Project already exists")]
    ProjectAlreadyExists,

    #[error("Project not found")]
    ProjectNotFound,

    #[error("Function not found")]
    FunctionNotFound,

    #[error("Function already exists")]
    FunctionAlreadyExists,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("{}", self);

        let (status, error_message) = match self {
            Error::Token(err) => match err.into_kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
                }
                jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                    (StatusCode::UNAUTHORIZED, "Invalid signature".to_string())
                }

                jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
                    (StatusCode::UNAUTHORIZED, "Invalid issuer".to_string())
                }
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    (StatusCode::UNAUTHORIZED, "Token expired".to_string())
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                ),
            },

            Error::UserNotRegistered => {
                (StatusCode::UNAUTHORIZED, "User not registered".to_string())
            }
            Error::ProjectNotFound => (StatusCode::NOT_FOUND, "Project not found".to_string()),
            Error::FunctionNotFound => (StatusCode::NOT_FOUND, "Function not found".to_string()),

            Error::ProjectAlreadyExists => {
                (StatusCode::CONFLICT, "Project already exists".to_string())
            }
            Error::FunctionAlreadyExists => {
                (StatusCode::CONFLICT, "Function already exists".to_string())
            }

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };
        (status, Json(ErrorDTO::new(&error_message))).into_response()
    }
}
