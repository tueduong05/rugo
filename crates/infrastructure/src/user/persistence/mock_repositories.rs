use std::{collections::HashMap, sync::Mutex};

use business::domain::user::{
    entities::User, error::DomainError, repositories::UserRepository,
    value_objects::login_identifier::LoginIdentifier,
};

pub struct MockUserRepository {
    users: Mutex<HashMap<String, User>>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for MockUserRepository {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let mut users = self.users.lock().unwrap();

        if users.values().any(|u| u.email == user.email) {
            return Err(DomainError::EmailTaken);
        }
        if users.values().any(|u| u.username == user.username) {
            return Err(DomainError::UsernameTaken);
        }

        users.insert(user.id.to_string(), user.to_owned().clone());

        Ok(())
    }

    async fn find_by_identifier(
        &self,
        identifier: &LoginIdentifier,
    ) -> Result<Option<User>, DomainError> {
        todo!()
    }
}
