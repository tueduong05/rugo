use std::fmt;

use url::Url;

use crate::domain::{common::error::BaseDomainError, link::error::LinkDomainError};

#[derive(Clone)]
pub struct OriginalLink(String);

impl OriginalLink {
    pub fn new(value: String) -> Result<Self, LinkDomainError> {
        if value.is_empty() || value != value.trim() || value.len() > 4096 {
            return Err(BaseDomainError::Unexpected(
                "Original link does not meet domain requirements".into(),
            )
            .into());
        }

        Url::parse(&value).map_err(|_| LinkDomainError::InvalidLink)?;

        Ok(Self(value))
    }
}

impl fmt::Display for OriginalLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
