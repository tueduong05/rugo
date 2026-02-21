#[derive(Debug, thiserror::Error)]
pub enum BaseDomainError {
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
