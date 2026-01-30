use crate::{application::error::AppError, domain::user::value_objects::user_id::UserId};

pub struct AccessToken {
    pub token: String,
    pub expires_at: u64,
}

pub struct RefreshToken {
    pub token: String,
    pub expires_at: u64,
}

pub trait TokenService {
    fn issue_access_token(&self, id: UserId) -> Result<AccessToken, AppError>;
}
