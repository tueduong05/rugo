use crate::domain::user::{
    entity::User,
    error::DomainError,
    value_objects::{
        email::Email, login_identifier::LoginIdentifier, user_id::UserId, username::Username,
    },
};

pub trait UserRepository {
    // Register
    fn exists_by_username(&self, username: &Username) -> Result<bool, DomainError>;
    fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError>;
    fn save(&self, user: User) -> Result<(), DomainError>;

    // Login
    fn find_by_identifier(&self, identifier: &LoginIdentifier)
    -> Result<Option<User>, DomainError>;

    // Refresh
    fn find_by_refresh_token(&self, refresh_token: &str) -> Result<Option<User>, DomainError>;
    fn update_refresh_token(
        &self,
        user_id: UserId,
        token: Option<String>,
    ) -> Result<(), DomainError>;
}
