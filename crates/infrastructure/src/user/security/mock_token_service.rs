use business::{
    application::user::{
        error::AppError,
        services::token_service::{TokenService, Tokens},
    },
    domain::user::value_objects::user_id::UserId,
};

pub struct MockTokenService;

#[async_trait::async_trait]
impl TokenService for MockTokenService {
    async fn issue_tokens(&self, _id: UserId) -> Result<Tokens, AppError> {
        Ok(Tokens {
            access_token: "mock_access_token".to_string(),
            expires_in: 900,
            refresh_token: "mock_refresh_token".to_string(),
        })
    }

    async fn refresh_session(&self, _token: String) -> Result<Tokens, AppError> {
        Ok(Tokens {
            access_token: "new_mock_access_token".to_string(),
            expires_in: 900,
            refresh_token: "new_mock_refresh_token".to_string(),
        })
    }

    async fn revoke_token(&self, _token: String) -> Result<(), AppError> {
        Ok(())
    }

    async fn revoke_all(&self, _user_id: UserId) -> Result<(), AppError> {
        Ok(())
    }
}
