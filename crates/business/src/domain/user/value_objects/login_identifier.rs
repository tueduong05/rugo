use crate::domain::user::{
    error::DomainError,
    value_objects::{email::Email, username::Username},
};

pub enum LoginIdentifier {
    Username(Username),
    Email(Email),
}

impl LoginIdentifier {
    pub fn parse(identifier: &str) -> Result<Self, DomainError> {
        if identifier.contains('@') {
            Ok(LoginIdentifier::Email(Email::new(identifier.into())?))
        } else {
            Ok(LoginIdentifier::Username(Username::new(identifier.into())?))
        }
    }
}
