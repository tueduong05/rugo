use crate::domain::user::value_objects::{email::Email, username::Username};

pub enum LoginIdentifier {
    Username(Username),
    Email(Email),
}
