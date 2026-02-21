use crate::domain::user::{
    entities::{RefreshToken, User},
    error::UserDomainError,
    value_objects::{login_identifier::LoginIdentifier, user_id::UserId},
};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    // Register
    async fn save(&self, user: &User) -> Result<(), UserDomainError>;

    // Login
    async fn find_by_identifier(
        &self,
        identifier: &LoginIdentifier,
    ) -> Result<Option<User>, UserDomainError>;

    // Get me
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UserDomainError>;
}

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn save(
        &self,
        session: RefreshToken,
        old_version: Option<u64>,
    ) -> Result<(), UserDomainError>;

    async fn find_by_token(&self, token: &str) -> Result<RefreshToken, UserDomainError>;

    async fn revoke(&self, user_id: &UserId, token: &str) -> Result<(), UserDomainError>;

    async fn revoke_all(&self, user_id: &UserId) -> Result<(), UserDomainError>;
}
