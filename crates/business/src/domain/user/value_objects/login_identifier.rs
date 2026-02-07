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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_login_identifier() {
        let test_cases = vec![
            ("mot+hai@example.com", "email"),
            ("mothai@example.com", "email"),
            ("mot_hai", "username"),
            ("mothai12", "username"),
        ];

        for (input, expected_type) in test_cases {
            let result = LoginIdentifier::parse(input)
                .expect(&format!("Should parse successfully: {}", input));

            match expected_type {
                "email" => assert!(
                    matches!(result, LoginIdentifier::Email(_)),
                    "Expected Email variant for: {}",
                    input
                ),
                "username" => assert!(
                    matches!(result, LoginIdentifier::Username(_)),
                    "Expected Username variant for: {}",
                    input
                ),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_invalid_login_identifier() {
        let invalid_email = "invalid@email";
        let result = LoginIdentifier::parse(invalid_email);
        assert!(
            result.is_err(),
            "Should fail because Email validation fails"
        );

        let result = LoginIdentifier::parse("");
        assert!(
            result.is_err(),
            "Should fail because Username validation fails"
        );
    }
}
