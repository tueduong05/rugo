use std::sync::Arc;

use validator::Validate;

use crate::{
    application::user::{
        dtos::auth_response::AuthResponse,
        error::AppError,
        services::token_service::{self, TokenService},
        use_cases::login::{LoginUseCase, request::LoginRequest},
    },
    domain::user::{
        error::DomainError, repositories::UserRepository,
        services::password_services::PasswordHasher,
        value_objects::login_identifier::LoginIdentifier,
    },
};

pub struct LoginInteractor {
    user_repo: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_service: Arc<dyn TokenService>,
}

impl LoginInteractor {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        token_service: Arc<dyn TokenService>,
    ) -> Self {
        Self {
            user_repo,
            password_hasher,
            token_service,
        }
    }
}

#[async_trait::async_trait]
impl LoginUseCase for LoginInteractor {
    async fn execute(&self, req: LoginRequest) -> Result<AuthResponse, AppError> {
        req.validate().map_err(AppError::from)?;

        let identifier =
            LoginIdentifier::parse(&req.identifier).map_err(|_| DomainError::InvalidCredentials)?;

        let user = self
            .user_repo
            .find_by_identifier(&identifier)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        if !self
            .password_hasher
            .verify(&req.password, &user.hashed_password)
        {
            return Err(DomainError::InvalidCredentials.into());
        }

        let tokens = self.token_service.issue_tokens(user.id).await?;

        Ok(AuthResponse {
            user_profile: user.into(),
            access_token: tokens.access_token,
            token_type: "Bearer".into(),
            expires_in: tokens.expires_in,
            refresh_token: tokens.refresh_token,
        })
    }
}
