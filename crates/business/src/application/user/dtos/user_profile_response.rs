use uuid::Uuid;

pub struct UserProfileResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub status: String,
}
