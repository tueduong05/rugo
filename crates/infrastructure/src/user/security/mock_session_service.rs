use business::{
    application::{
        common::services::session_service::{SessionService, Tokens},
        error::AppError,
    },
    domain::{
        common::{error::BaseDomainError, value_objects::user_id::UserId},
        user::error::UserDomainError,
    },
};
use sqlx::types::Uuid;

pub struct MockSessionService;

#[async_trait::async_trait]
impl SessionService for MockSessionService {
    async fn start_session(&self, _id: UserId) -> Result<Tokens, AppError> {
        Ok(Tokens {
            access_token: "mock_access_token".to_string(),
            expires_in: 900,
            refresh_token: "mock_refresh_token".to_string(),
        })
    }

    async fn rotate_session(&self, _token: &str) -> Result<Tokens, AppError> {
        Ok(Tokens {
            access_token: "new_mock_access_token".to_string(),
            expires_in: 900,
            refresh_token: "new_mock_refresh_token".to_string(),
        })
    }

    async fn end_session(&self, _user_id: UserId, _token: &str) -> Result<(), AppError> {
        Ok(())
    }

    async fn end_all_sessions(&self, _user_id: UserId) -> Result<(), AppError> {
        Ok(())
    }

    async fn authenticate(&self, access_token: &str) -> Result<UserId, AppError> {
        match access_token {
            "mock_access_token" => {
                let uuid =
                    Uuid::parse_str("00000000-0000-0000-0000-000000000000").map_err(|_| {
                        UserDomainError::from(BaseDomainError::Unexpected(
                            "Invalid mock UserId".into(),
                        ))
                    })?;

                Ok(UserId::from(uuid))
            }

            "mock_expired_access_token" => {
                Err(UserDomainError::from(BaseDomainError::SessionExpired).into())
            }
            _ => Err(UserDomainError::from(BaseDomainError::InvalidSession).into()),
        }
    }
}
