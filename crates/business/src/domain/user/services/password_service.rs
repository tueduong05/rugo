use crate::domain::user::value_objects::password::Password;

pub trait PasswordService {
    fn hash(&self, password: &Password) -> String;
    fn verify(&self, password: &Password, password_hash: &str) -> bool;
}
