use std::sync::LazyLock;

use regex::Regex;

use crate::domain::user::error::DomainError;

pub static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$").expect("Invalid email regex"));

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.is_empty()
            || value != value.trim()
            || value.chars().count() > 256
            || !EMAIL_REGEX.is_match(&value)
        {
            return Err(DomainError::Unexpected(
                "Email does not meet domain requirements".into(),
            ));
        }

        Ok(Self(value))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
