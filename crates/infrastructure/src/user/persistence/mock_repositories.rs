use std::{collections::HashMap, sync::Mutex};

use business::domain::{
    common::error::BaseDomainError,
    user::{
        entities::{RefreshToken, User},
        error::UserDomainError,
        repositories::{SessionRepository, UserRepository},
        value_objects::{
            email::Email, login_identifier::LoginIdentifier, user_id::UserId, username::Username,
        },
    },
};

#[derive(Default)]
pub struct MockUserRepository {
    users: Mutex<HashMap<UserId, User>>,
    emails: Mutex<HashMap<Email, UserId>>,
    usernames: Mutex<HashMap<Username, UserId>>,
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
    async fn save(&self, user: &User) -> Result<(), UserDomainError> {
        let mut users = self.users.lock().unwrap();
        let mut emails = self.emails.lock().unwrap();
        let mut usernames = self.usernames.lock().unwrap();

        if emails.contains_key(&user.email) {
            return Err(UserDomainError::EmailTaken);
        }
        if usernames.contains_key(&user.username) {
            return Err(UserDomainError::UsernameTaken);
        }

        emails.insert(user.email.clone(), user.id);
        usernames.insert(user.username.clone(), user.id);
        users.insert(user.id, user.clone());

        Ok(())
    }

    async fn find_by_identifier(
        &self,
        identifier: &LoginIdentifier,
    ) -> Result<Option<User>, UserDomainError> {
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

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<User>, UserDomainError> {
        let users = self
            .users
            .lock()
            .map_err(|e| UserDomainError::Base(BaseDomainError::Infrastructure(e.to_string())))?;

        let user = users.get(user_id).cloned();

        Ok(user)
    }
}

#[derive(Default)]
pub struct MockSessionRepository {
    sessions: Mutex<HashMap<String, RefreshToken>>,
}

impl MockSessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl SessionRepository for MockSessionRepository {
    async fn save(
        &self,
        session: RefreshToken,
        old_version: Option<u64>,
    ) -> Result<(), UserDomainError> {
        let mut sessions = self.sessions.lock().unwrap();

        match old_version {
            None => {
                if sessions.contains_key(&session.token) {
                    return Err(UserDomainError::Base(BaseDomainError::Infrastructure(
                        "Token already exists".into(),
                    )));
                }
                sessions.insert(session.token.clone(), session);
            }

            Some(expected) => {
                let existing = sessions
                    .get_mut(&session.token)
                    .ok_or(UserDomainError::InvalidSession)?;

                if existing.version != expected {
                    return Err(UserDomainError::ConcurrencyError);
                }

                *existing = session;
            }
        }

        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<RefreshToken, UserDomainError> {
        let sessions = self.sessions.lock().unwrap();

        sessions
            .get(token)
            .cloned()
            .ok_or(UserDomainError::InvalidSession)
    }

    async fn revoke(&self, user_id: &UserId, token: &str) -> Result<(), UserDomainError> {
        let mut sessions = self.sessions.lock().unwrap();

        let session = sessions
            .get_mut(token)
            .ok_or(UserDomainError::InvalidSession)?;

        if &session.user_id != user_id {
            return Err(UserDomainError::AccessDenied);
        }

        session.is_revoked = true;

        Ok(())
    }

    async fn revoke_all(&self, user_id: &UserId) -> Result<(), UserDomainError> {
        let mut sessions = self.sessions.lock().unwrap();

        for session in sessions.values_mut() {
            if &session.user_id == user_id {
                session.is_revoked = true;
            }
        }

        Ok(())
    }
}
