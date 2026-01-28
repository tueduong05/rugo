#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    // Register Errors
    #[error("Username is taken")]
    UsernameTaken,
    #[error("Email is taken")]
    EmailTaken,

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

    // Unexpected
    #[error("Internal domain service error: {0}")]
    Internal(String),
}
