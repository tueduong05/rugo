use std::sync::LazyLock;

use regex::Regex;

use crate::domain::user::error::DomainError;

pub static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?-u)^[a-zA-Z0-9_\-]+$").expect("Invalid username regex"));

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    pub fn new(value: String) -> Result<Self, DomainError> {
        let len = value.len();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_valid() {
        let names = vec!["_mothaiba", "Mot23", "mot-hai-ba"];
        for name in names {
            let result = Username::new(name.to_string());
            assert!(result.is_ok(), "Should be valid: {}", name);
            assert_eq!(result.unwrap().into_inner(), name);
        }
    }

    #[test]
    fn test_username_too_short() {
        let result = Username::new("ba".to_string());
        assert!(result.is_err(), "Should fail: too short");
    }

    #[test]
    fn test_username_too_long() {
        let result = Username::new("thequickbrownfoxjumpsoverthelazydog".to_string());
        assert!(result.is_err(), "Should fail: too long");
    }

    #[test]
    fn test_username_whitespace() {
        assert!(Username::new(" mothaiba".to_string()).is_err());
        assert!(Username::new("mothaiba ".to_string()).is_err());
        assert!(Username::new("mot hai".to_string()).is_err());
    }

    #[test]
    fn test_username_invalid_characters() {
        assert!(Username::new("mot@hai".to_string()).is_err());
        assert!(Username::new("mot.hai".to_string()).is_err());
    }

    #[test]
    fn test_username_non_ascii() {
        assert!(Username::new("🦀rust".to_string()).is_err());
        assert!(Username::new("mộthaiba".to_string()).is_err());
    }
}
