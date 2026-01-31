use crate::domain::user::value_objects::{
    email::Email, hashed_password::HashedPassword, user_id::UserId, user_status::UserStatus,
    username::Username,
};

pub struct User {
    id: UserId,
    username: Username,
    email: Email,
    hashed_password: HashedPassword,
    status: UserStatus,
}

impl User {
    pub fn new(
        id: UserId,
        username: Username,
        email: Email,
        hashed_password: HashedPassword,
        status: UserStatus,
    ) -> Self {
        Self {
            id,
            username,
            email,
            hashed_password,
            status,
        }
    }
}
