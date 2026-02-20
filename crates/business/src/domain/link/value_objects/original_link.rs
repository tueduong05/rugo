use url::Url;

use crate::domain::link::error::DomainError;

pub struct OriginalLink(String);

impl OriginalLink {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.is_empty()
            || value != value.trim()
            || value.len() > 4096
            || Url::parse(&value).is_err()
        {
            return Err(DomainError::Unexpected(
                "Original link does not meet domain requirements".into(),
            ));
        }

        Ok(Self(value))
    }
}
