#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Short code already exists")]
    ShortCodeAlreadyExists,
    #[error("Invalid link")]
    InvalidLink,

    #[error("Invalid short code")]
    InvalidShortCode,
    #[error("Link expired")]
    LinkExpired,
    #[error("Wrong password")]
    WrongPassword,

    // Infrastructure
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    // Unexpected
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
