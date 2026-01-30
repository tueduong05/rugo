use crate::domain::user::value_objects::{
    email::Email, user_id::UserId, user_status::UserStatus, username::Username,
};

pub struct UserProfileResponse {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
    pub status: UserStatus,
}
