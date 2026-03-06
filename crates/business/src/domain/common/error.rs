#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum BaseDomainError {
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

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Concurrency error")]
    ConcurrencyError,

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
