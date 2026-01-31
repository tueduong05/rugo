#[derive(Debug, thiserror::Error)]
pub enum DomainError {
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

    // Session & Authorization Errors
    #[error("Invalid access token")]
    InvalidAccessToken,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Session expired")]
    SessionExpired,

    // Refresh Errors
    #[error("Invalid refresh token")]
    InvalidRefreshToken,
    #[error("Refresh token revoked")]
    RefreshTokenRevoked,

    // Infrastructure
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    // Unexpected
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
