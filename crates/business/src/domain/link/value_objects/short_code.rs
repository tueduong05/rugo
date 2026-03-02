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
