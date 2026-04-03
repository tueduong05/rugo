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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_original_link_valid() {
        let valid_links = vec![
            "https://example.com/path?x=1",
            "http://localhost:3000/test",
            "https://sub.example.org/a/b/c",
        ];

        for link in valid_links {
            assert!(
                OriginalLink::new(link.to_string()).is_ok(),
                "Should be valid: {}",
                link
            );
        }
    }

    #[test]
    fn test_original_link_invalid_empty() {
        assert!(OriginalLink::new("".into()).is_err());
    }

    #[test]
    fn test_original_link_invalid_trimmed() {
        assert!(OriginalLink::new(" https://example.com".into()).is_err());
        assert!(OriginalLink::new("https://example.com ".into()).is_err());
    }

    #[test]
    fn test_original_link_invalid_url_format() {
        let invalid_links = vec!["not-a-url", "example.com", "https://"];

        for link in invalid_links {
            let result = OriginalLink::new(link.to_string());

            assert!(matches!(result, Err(LinkDomainError::InvalidLink)));
        }
    }

    #[test]
    fn test_original_link_invalid_too_long() {
        let long_url = format!("https://example.com/{}", "a".repeat(5000));

        assert!(OriginalLink::new(long_url).is_err());
    }
}
