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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashed_password_valid() {
        let valid_passwords = vec!["$argon2id$v=19$m=65536,t=3,p=1$abc$def", "hashed-value"];

        for password in valid_passwords {
            assert!(
                HashedPassword::new(password.to_string()).is_ok(),
                "Should be valid: {}",
                password
            );
        }
    }

    #[test]
    fn test_hashed_password_invalid_empty() {
        assert!(HashedPassword::new("".into()).is_err());
    }
}
