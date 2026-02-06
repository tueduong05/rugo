use crate::domain::user::value_objects::user_id::UserId;

pub struct GetMeCommand {
    pub user_id: UserId,
}

impl GetMeCommand {
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
    }
}
