use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum AppErr {
    #[error("Missing Authorization Headers!")]
    MissingAuthorization,
    #[error("Invalid Credentials!")]
    InvalidCredentials,
    #[error("User Does Not Exist")]
    UserDoesNotExist,
    #[error("Asset does not exist!")]
    AssetDoesNotExist,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error("This username is already registered")]
    UsernameTaken,
    #[error(transparent)]
    Template(#[from] askama::Error),
}
#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppErr {
    fn into_response(self) -> axum::response::Response {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };
        let status = match self {
            Self::UsernameTaken | Self::MissingAuthorization => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::UserDoesNotExist | Self::AssetDoesNotExist => StatusCode::NOT_FOUND,
            Self::Template(_) | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(error_response)).into_response()
    }
}
