use std::sync::Arc;

use axum::{extract::FromRequestParts, http::request::Parts};
use business::{
    application::{error::AppError, user::services::session_service::SessionService},
    domain::user::{error::UserDomainError, value_objects::user_id::UserId},
};

use crate::user::http::error::HttpError;

pub struct AuthenticatedUser(pub UserId);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let session_service = parts
            .extensions
            .get::<Arc<dyn SessionService>>()
            .expect("SessionService missing");

        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .filter(|h| h.starts_with("Bearer "))
            .map(|h| &h[7..])
            .ok_or(AppError::User(UserDomainError::InvalidSession))?;

        let user_id = session_service.authenticate(auth_header).await?;

        Ok(AuthenticatedUser(user_id))
    }
}
