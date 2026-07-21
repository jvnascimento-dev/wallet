use crate::{app::AppState, error::AppErr};
use axum::{extract::FromRequestParts, http::header::AUTHORIZATION};

const ADMIN_SECRET_KEY: &str = "adm-here";
pub struct Admin;

impl FromRequestParts<AppState> for Admin {
    type Rejection = AppErr;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Some(auth) = parts.headers.get(AUTHORIZATION) else {
            return Err(AppErr::MissingAuthorization);
        };
        if auth == ADMIN_SECRET_KEY {
            Ok(Admin)
        } else {
            Err(AppErr::InvalidCredentials)
        }
    }
}
