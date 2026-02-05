use std::sync::LazyLock;

use regex::Regex;

use crate::domain::user::error::DomainError;

pub static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_\-]+$").expect("Invalid username regex"));

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    pub fn new(value: String) -> Result<Self, DomainError> {
        let len = value.chars().count();

        if value != value.trim() || !(3..=20).contains(&len) || !USERNAME_REGEX.is_match(&value) {
            return Err(DomainError::Unexpected(
                "Username does not meet domain requirements".into(),
            ));
        }

        Ok(Self(value))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
