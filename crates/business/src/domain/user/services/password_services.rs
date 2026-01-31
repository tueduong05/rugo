use crate::domain::user::value_objects::hashed_password::HashedPassword;

pub trait PasswordPolicy {
    fn validate(&self, password: &str) -> bool;
}

pub trait PasswordHasher {
    fn hash(&self, password: &str) -> String;
    fn verify(&self, password: &str, hash: HashedPassword) -> bool;
}
