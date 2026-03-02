use serde::Serialize;
use uuid::Uuid;

use crate::domain::user::entities::User;

#[derive(Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct UserProfileResponse {
    #[cfg_attr(feature = "openapi", schema(value_type = String, format = "uuid"))]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub status: String,
}

impl From<User> for UserProfileResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.value(),
            username: user.username.to_string(),
            email: user.email.to_string(),
            status: user.status.to_string(),
        }
    }
}
