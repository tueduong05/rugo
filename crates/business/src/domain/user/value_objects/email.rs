use std::{fmt, sync::LazyLock};

use regex::Regex;

use crate::domain::{common::error::BaseDomainError, user::error::UserDomainError};

pub static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^[\w!#$%&'*+/=?^_`{|}~.-]+@([\w-]+\.)+[\w-]{2,}$")
        .expect("Invalid email regex")
});

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, UserDomainError> {
        if value.is_empty()
            || value != value.trim()
            || value.chars().count() > 256
            || !EMAIL_REGEX.is_match(&value)
        {
            return Err(BaseDomainError::Unexpected(
                "Email does not meet domain requirements".into(),
            )
            .into());
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_valid() {
        let valid_emails = vec![
            "mothai@example.com",
            "mot.hai@example.com",
            "mot.hai+ba@example.com",
            "mot.hai-ba@example.com",
            "mot-hai@ba.example.com",
            "mot.hai@example.io",
            "mộthai@example.com",
        ];

        for email in valid_emails {
            assert!(
                Email::new(email.to_string()).is_ok(),
                "Should be valid: {}",
                email
            );
        }
    }

    #[test]
    fn test_email_invalid() {
        let invalid_emails = vec![
            "mothai.com",          // Missing @
            "mothai@",             // Missing domain
            "@example.com",        // Missing local part
            "mothai @example.com", // Internal space
            "mothai@@example.com", // Double @
            "mot.hai@example.c",   // Domain extension too short
        ];

        for email in invalid_emails {
            assert!(
                Email::new(email.to_string()).is_err(),
                "Should be invalid: {}",
                email
            );
        }
    }

    #[test]
    fn test_email_trim() {
        assert!(Email::new(" mothai@example.com".into()).is_err());
        assert!(Email::new("mothai@example.com ".into()).is_err());
    }

    #[test]
    fn test_email_length() {
        let long_local = "a".repeat(250);
        let long_email = format!("{}@example.com", long_local);

        assert!(
            Email::new(long_email).is_err(),
            "Should fail for being over 256 chars"
        );
    }

    #[test]
    fn test_email_empty() {
        assert!(Email::new("".into()).is_err(), "Empty email should fail");
    }
}
