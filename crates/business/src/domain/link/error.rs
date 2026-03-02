use crate::domain::common::error::BaseDomainError;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum LinkDomainError {
    #[error(transparent)]
    Base(#[from] BaseDomainError),

    // Post Link Errors
    #[error("Short code already exists")]
    ShortCodeAlreadyExists,
    #[error("Short code collision limit reached")]
    ShortCodeCollisionLimitReached,
    #[error("Invalid link")]
    InvalidLink,

    // Get Link Errors
    #[error("Link expired")]
    LinkExpired,
    #[error("Link click limit reached")]
    LinkClickLimitReached,
    #[error("Link not active")]
    LinkNotActive,
    #[error("Password required")]
    PasswordRequired,
    #[error("Wrong password")]
    WrongPassword,
}
