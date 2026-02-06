use std::sync::Arc;

use axum::{extract::FromRequestParts, http::request::Parts};
use business::{
    application::user::{error::AppError, services::token_service::TokenService},
    domain::user::{error::DomainError, value_objects::user_id::UserId},
};

use crate::user::http::error::HttpError;

pub struct AuthenticatedUser(pub UserId);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token_service = parts
            .extensions
            .get::<Arc<dyn TokenService>>()
            .expect("TokenService missing");

        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .filter(|h| h.starts_with("Bearer "))
            .map(|h| &h[7..])
            .ok_or_else(|| AppError::Domain(DomainError::InvalidAccessToken))?;

        let user_id = token_service.verify_access_token(&auth_header).await?;

        Ok(AuthenticatedUser(user_id))
    }
}
