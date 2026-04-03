use std::{fmt, sync::LazyLock};

use regex::Regex;

use crate::domain::{common::error::BaseDomainError, link::error::LinkDomainError};

pub static SHORTCODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?-u)^[a-zA-Z0-9_\-]+$").expect("Invalid username regex"));

#[derive(Clone)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new(value: String) -> Result<Self, LinkDomainError> {
        if value != value.trim()
            || !(3..=20).contains(&value.len())
            || !SHORTCODE_REGEX.is_match(&value)
        {
            return Err(BaseDomainError::Unexpected(
                "Short code does not meet domain requirements".into(),
            )
            .into());
        }

        Ok(Self(value))
    }
}

impl fmt::Display for ShortCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_code_valid() {
        let valid_short_codes = vec![
            "abc".to_string(),
            "abc123".to_string(),
            "abc_123".to_string(),
            "abc-123".to_string(),
            "A1_b-2".to_string(),
            "a".repeat(20),
        ];

        for short_code in valid_short_codes {
            assert!(
                ShortCode::new(short_code.clone()).is_ok(),
                "Should be valid: {}",
                short_code
            );
        }
    }

    #[test]
    fn test_short_code_invalid_length() {
        let invalid_short_codes = vec![
            "ab".to_string(),
            "a".to_string(),
            "".to_string(),
            "a".repeat(21),
        ];

        for short_code in invalid_short_codes {
            assert!(
                ShortCode::new(short_code.clone()).is_err(),
                "Should be invalid length: {}",
                short_code
            );
        }
    }

    #[test]
    fn test_short_code_invalid_trim() {
        assert!(ShortCode::new(" abc".into()).is_err());
        assert!(ShortCode::new("abc ".into()).is_err());
    }

    #[test]
    fn test_short_code_invalid_characters() {
        let invalid_short_codes = vec!["abc!", "abc.def", "abc/123", "một-hai", "a b c"];

        for short_code in invalid_short_codes {
            assert!(
                ShortCode::new(short_code.to_string()).is_err(),
                "Should be invalid: {}",
                short_code
            );
        }
    }
}
