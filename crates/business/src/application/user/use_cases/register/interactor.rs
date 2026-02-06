use std::sync::Arc;

use chrono::Utc;

use crate::{
    application::user::{
        dtos::auth_response::AuthResponse,
        error::AppError,
        services::token_service::TokenService,
        use_cases::register::{RegisterUseCase, request::RegisterRequest},
    },
    domain::user::{
        entities::User,
        error::DomainError,
        repositories::UserRepository,
        services::password_services::{PasswordHasher, PasswordPolicy},
        value_objects::{
            email::Email, hashed_password::HashedPassword, user_id::UserId,
            user_status::UserStatus, username::Username,
        },
    },
};

pub struct RegisterInteractor {
    user_repo: Arc<dyn UserRepository>,
    password_policy: Arc<dyn PasswordPolicy>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_service: Arc<dyn TokenService>,
}

impl RegisterInteractor {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_policy: Arc<dyn PasswordPolicy>,
        password_hasher: Arc<dyn PasswordHasher>,
        token_service: Arc<dyn TokenService>,
    ) -> Self {
        Self {
            user_repo,
            password_policy,
            password_hasher,
            token_service,
        }
    }
}

#[async_trait::async_trait]
impl RegisterUseCase for RegisterInteractor {
    async fn execute(&self, req: RegisterRequest) -> Result<AuthResponse, AppError> {
        if !self.password_policy.validate(&req.password) {
            return Err(DomainError::PasswordTooWeak.into());
        }

        let username = Username::new(req.username)?;
        let email = Email::new(req.email)?;
        let hashed_password = HashedPassword::new(self.password_hasher.hash(&req.password))?;

        let user = User::new(
            UserId::generate(),
            username,
            email,
            hashed_password,
            UserStatus::Verified,
            Utc::now(),
        );

        self.user_repo.save(&user).await?;

        let tokens = self.token_service.issue_tokens(&user.id).await?;

        Ok(AuthResponse {
            user_profile: user.into(),
            access_token: tokens.access_token,
            token_type: "Bearer".into(),
            expires_in: tokens.expires_in,
            refresh_token: tokens.refresh_token,
        })
    }
}
