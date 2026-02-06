use crate::{application::user::error::AppError, domain::user::value_objects::user_id::UserId};

pub struct Tokens {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
}

#[async_trait::async_trait]
pub trait TokenService: Send + Sync {
    async fn issue_tokens(&self, id: &UserId) -> Result<Tokens, AppError>;

    async fn refresh_session(&self, refresh_token: &str) -> Result<Tokens, AppError>;

    async fn revoke_token(&self, user_id: &UserId, refresh_token: &str) -> Result<(), AppError>;

    async fn revoke_all(&self, user_id: &UserId) -> Result<(), AppError>;

    async fn verify_access_token(&self, access_token: &str) -> Result<UserId, AppError>;
}
