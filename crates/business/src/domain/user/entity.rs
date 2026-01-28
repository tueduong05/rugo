use crate::domain::user::value_objects::{
    email::Email, user_id::UserId, user_status::UserStatus, username::Username,
};

pub struct User {
    id: UserId,
    username: Username,
    email: Email,
    status: UserStatus,
}
