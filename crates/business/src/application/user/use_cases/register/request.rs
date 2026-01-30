use crate::domain::user::value_objects::{email::Email, password::Password, username::Username};

pub struct RegisterRequest {
    pub username: Username,
    pub email: Email,
    pub password: Password,
}
