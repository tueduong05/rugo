use crate::domain::common::error::BaseDomainError;

#[derive(Debug, thiserror::Error)]
pub enum LinkDomainError {
    #[error(transparent)]
    Base(#[from] BaseDomainError),

    #[error("Short code already exists")]
    ShortCodeAlreadyExists,
    #[error("Short code collision limit reached")]
    ShortCodeCollisionLimitReached,
    #[error("Invalid link")]
    InvalidLink,

    #[error("Invalid short code")]
    InvalidShortCode,
    #[error("Link expired")]
    LinkExpired,
    #[error("Wrong password")]
    WrongPassword,
}
