use crate::application::user::dtos::user_profile_response::UserProfileResponse;

pub struct AuthResponse {
    pub user_profile: UserProfileResponse,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
}
