use std::sync::Arc;

use chrono::Utc;

use crate::{
    application::{
        error::AppError,
        user::{
            common::auth_response::AuthResponse,
            services::session_service::SessionService,
            use_cases::register::{RegisterUseCase, request::RegisterRequest},
        },
    },
    domain::{
        common::{
            services::password_services::{PasswordHasher, PasswordPolicy},
            value_objects::hashed_password::HashedPassword,
        },
        user::{
            entities::User,
            error::UserDomainError,
            repositories::UserRepository,
            value_objects::{
                email::Email, user_id::UserId, user_status::UserStatus, username::Username,
            },
        },
    },
};

pub struct RegisterInteractor {
    user_repo: Arc<dyn UserRepository>,
    password_policy: Arc<dyn PasswordPolicy>,
    password_hasher: Arc<dyn PasswordHasher>,
    session_service: Arc<dyn SessionService>,
}

impl RegisterInteractor {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_policy: Arc<dyn PasswordPolicy>,
        password_hasher: Arc<dyn PasswordHasher>,
        session_service: Arc<dyn SessionService>,
    ) -> Self {
        Self {
            user_repo,
            password_policy,
            password_hasher,
            session_service,
        }
    }
}

#[async_trait::async_trait]
impl RegisterUseCase for RegisterInteractor {
    async fn execute(&self, req: RegisterRequest) -> Result<AuthResponse, AppError> {
        if !self.password_policy.validate(&req.password) {
            return Err(UserDomainError::PasswordTooWeak.into());
        }

        let username = Username::new(req.username)?;
        let email = Email::new(req.email)?;
        let hashed_password = HashedPassword::new(self.password_hasher.hash(&req.password))
            .map_err(|e| UserDomainError::Base(e))?;

        let user = User::new(
            UserId::generate(),
            username,
            email,
            hashed_password,
            UserStatus::Verified,
            Utc::now(),
        );

        self.user_repo.save(&user).await?;

        let tokens = self.session_service.start_session(&user.id).await?;

        Ok(AuthResponse {
            user_profile: user.into(),
            access_token: tokens.access_token,
            token_type: "Bearer".into(),
            expires_in: tokens.expires_in,
            refresh_token: tokens.refresh_token,
        })
    }
}
