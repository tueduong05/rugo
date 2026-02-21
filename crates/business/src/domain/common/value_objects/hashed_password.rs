use std::fmt;

use crate::domain::common::error::BaseDomainError;

#[derive(Clone)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn new(value: String) -> Result<Self, BaseDomainError> {
        if value.is_empty() {
            return Err(BaseDomainError::Unexpected(
                "Hashed password does not meet domain requirements".into(),
            ));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
