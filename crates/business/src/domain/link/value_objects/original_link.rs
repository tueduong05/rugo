use std::fmt;

use url::Url;

use crate::domain::link::error::DomainError;

pub struct OriginalLink(String);

impl OriginalLink {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.is_empty() || value != value.trim() || value.len() > 4096 {
            return Err(DomainError::Unexpected(
                "Original link does not meet domain requirements".into(),
            ));
        }

        Url::parse(&value).map_err(|_| DomainError::InvalidLink)?;

        Ok(Self(value))
    }
}

impl fmt::Display for OriginalLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
