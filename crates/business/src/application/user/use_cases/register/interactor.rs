use std::sync::Arc;

use validator::Validate;

use crate::{
    application::{
        error::AppError,
        user::{
            dtos::auth_response::AuthResponse,
            services::token_service::TokenService,
            use_cases::register::{RegisterUseCase, request::RegisterRequest},
        },
    },
    domain::user::{
        entity::User,
        error::DomainError,
        repository::UserRepository,
        services::password_services::{PasswordHasher, PasswordPolicy},
        value_objects::{
            email::Email, hashed_password::HashedPassword, user_id::UserId,
            user_status::UserStatus, username::Username,
        },
    },
};

struct RegisterInteractor {
    user_repo: Arc<dyn UserRepository>,
    password_policy: Arc<dyn PasswordPolicy>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_service: Arc<dyn TokenService>,
}

impl RegisterUseCase for RegisterInteractor {
    async fn execute(&self, req: RegisterRequest) -> Result<AuthResponse, AppError> {
        req.validate().map_err(AppError::from)?;

        if self.password_policy.validate(&req.password) == false {
            return Err(DomainError::PasswordTooWeak.into());
        }

        let username = Username::new(req.username)?;
        let email = Email::new(req.email)?;

        if self.user_repo.exists_by_username(&username).await? {
            return Err(DomainError::UsernameTaken.into());
        }
        if self.user_repo.exists_by_email(&email).await? {
            return Err(DomainError::EmailTaken.into());
        }

        let hashed_password = HashedPassword::new(self.password_hasher.hash(&req.password))?;

        let user = User::new(
            UserId::generate(),
            username,
            email,
            hashed_password,
            UserStatus::Verified,
        );

        todo!()
    }
}
