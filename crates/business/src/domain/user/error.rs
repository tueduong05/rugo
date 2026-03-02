use crate::domain::common::error::BaseDomainError;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum UserDomainError {
    #[error(transparent)]
    Base(#[from] BaseDomainError),

    // Register Errors
    #[error("Username is taken")]
    UsernameTaken,
    #[error("Email is taken")]
    EmailTaken,
    #[error("Password too weak")]
    PasswordTooWeak,

    // Login Errors
    #[error("Invalid identifier or password")]
    InvalidCredentials,
    #[error("Email not verified")]
    EmailNotVerified,
    #[error("Account locked due to too many failed attempts")]
    AccountLocked,
    #[error("Account disabled")]
    AccountDisabled,

    // Session Errors
    #[error("Access denied")]
    AccessDenied,
    #[error("Invalid session")]
    InvalidSession,
    #[error("Session already used")]
    SessionAlreadyUsed,
    #[error("Session expired")]
    SessionExpired,
    #[error("Session revoked")]
    SessionRevoked,
}
