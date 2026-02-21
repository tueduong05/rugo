use business::domain::common::{
    services::password_services::PasswordHasher, value_objects::hashed_password::HashedPassword,
};

pub struct MockPasswordHasher;
impl PasswordHasher for MockPasswordHasher {
    fn hash(&self, password: &str) -> String {
        format!("hashed_{}", password)
    }

    fn verify(&self, password: &str, hash: &HashedPassword) -> bool {
        hash.as_str() == format!("hashed_{}", password)
    }
}
