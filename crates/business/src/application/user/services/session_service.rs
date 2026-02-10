use crate::{application::user::error::AppError, domain::user::value_objects::user_id::UserId};

pub struct Tokens {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
}

#[async_trait::async_trait]
pub trait SessionService: Send + Sync {
    async fn start_session(&self, id: &UserId) -> Result<Tokens, AppError>;

    async fn rotate_session(&self, refresh_token: &str) -> Result<Tokens, AppError>;

    async fn end_session(&self, user_id: &UserId, refresh_token: &str) -> Result<(), AppError>;

    async fn end_all_sessions(&self, user_id: &UserId) -> Result<(), AppError>;

    async fn authenticate(&self, access_token: &str) -> Result<UserId, AppError>;
}
