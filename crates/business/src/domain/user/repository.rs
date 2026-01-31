use crate::domain::user::{
    entity::User,
    error::DomainError,
    value_objects::{
        email::Email, login_identifier::LoginIdentifier, user_id::UserId, username::Username,
    },
};

#[async_trait::async_trait]
pub trait UserRepository {
    // Register
    async fn exists_by_username(&self, username: &Username) -> Result<bool, DomainError>;
    async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;

    // Login
    async fn find_by_identifier(
        &self,
        identifier: &LoginIdentifier,
    ) -> Result<Option<User>, DomainError>;

    // Refresh
    async fn find_by_refresh_token(&self, refresh_token: &str)
    -> Result<Option<User>, DomainError>;
    async fn update_refresh_token(
        &self,
        user_id: &UserId,
        refresh_token: Option<String>,
    ) -> Result<(), DomainError>;
}
