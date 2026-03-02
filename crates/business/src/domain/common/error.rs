#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum BaseDomainError {
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Concurrency error")]
    ConcurrencyError,

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
