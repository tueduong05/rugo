use std::{collections::HashMap, sync::Mutex};

use business::domain::user::{
    entities::User,
    error::DomainError,
    repositories::UserRepository,
    value_objects::{email::Email, login_identifier::LoginIdentifier, username::Username},
};

#[derive(Default)]
pub struct MockUserRepository {
    users: Mutex<HashMap<String, User>>,
    emails: Mutex<HashMap<Email, String>>,
    usernames: Mutex<HashMap<Username, String>>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            emails: Mutex::new(HashMap::new()),
            usernames: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for MockUserRepository {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let mut users = self.users.lock().unwrap();
        let mut emails = self.emails.lock().unwrap();
        let mut usernames = self.usernames.lock().unwrap();

        if emails.contains_key(&user.email) {
            return Err(DomainError::EmailTaken);
        }
        if usernames.contains_key(&user.username) {
            return Err(DomainError::UsernameTaken);
        }

        let id_str = user.id.to_string();

        emails.insert(user.email.clone(), id_str.clone());
        usernames.insert(user.username.clone(), id_str.clone());
        users.insert(id_str, user.clone());

        Ok(())
    }

    async fn find_by_identifier(
        &self,
        identifier: &LoginIdentifier,
    ) -> Result<Option<User>, DomainError> {
        let users = self.users.lock().unwrap();

        let user = match identifier {
            LoginIdentifier::Email(email) => {
                let emails = self.emails.lock().unwrap();
                emails.get(email).and_then(|id| users.get(id))
            }
            LoginIdentifier::Username(username) => {
                let usernames = self.usernames.lock().unwrap();
                usernames.get(username).and_then(|id| users.get(id))
            }
        };

        Ok(user.cloned())
    }
}
