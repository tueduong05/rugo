#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    // Resource Errors
    #[error("User not found")]
    UserNotFound,
    #[error("Concurrency error")]
    ConcurrencyError,

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
    #[error("Session expired")]
    SessionExpired,
    #[error("Session revoked")]
    SessionRevoked,

    // Infrastructure
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    // Unexpected
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
