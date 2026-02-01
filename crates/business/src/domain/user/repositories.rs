use crate::domain::user::{
    entities::{RefreshToken, User},
    error::DomainError,
    value_objects::login_identifier::LoginIdentifier,
};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    // Register
    async fn save(&self, user: &User) -> Result<(), DomainError>;

    // Login
    async fn find_by_identifier(
        &self,
        identifier: &LoginIdentifier,
    ) -> Result<Option<User>, DomainError>;
}

#[async_trait::async_trait]
pub trait TokenRepository {
    async fn save(&self, refresh_token: RefreshToken) -> Result<(), DomainError>;
}
