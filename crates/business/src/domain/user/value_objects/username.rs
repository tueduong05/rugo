use std::sync::LazyLock;

use regex::Regex;

use crate::domain::user::error::DomainError;

pub static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_\-]+$").expect("Invalid username regex"));

pub struct Username(String);

impl Username {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value != value.trim()
            || value.len() < 3
            || value.len() > 20
            || !USERNAME_REGEX.is_match(&value)
        {
            return Err(DomainError::Unexpected(
                "Username does not meet domain requirements".into(),
            ));
        }

        Ok(Self(value))
    }
}
