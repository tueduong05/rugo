use serde::Deserialize;
use validator::Validate;

use crate::domain::user::value_objects::user_id::UserId;

#[derive(Deserialize, Validate)]
pub struct LogoutRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

pub struct LogoutCommand {
    pub user_id: UserId,
    pub refresh_token: String,
}

impl LogoutCommand {
    pub fn new(user_id: UserId, refresh_token: String) -> Self {
        Self {
            user_id,
            refresh_token,
        }
    }
}
