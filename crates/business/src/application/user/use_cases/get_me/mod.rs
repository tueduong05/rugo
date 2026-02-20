use crate::application::user::{
    common::user_profile_response::UserProfileResponse, error::AppError,
    use_cases::get_me::command::GetMeCommand,
};

pub mod command;
pub mod interactor;

#[async_trait::async_trait]
pub trait GetMeUseCase: Send + Sync {
    async fn execute(&self, command: GetMeCommand) -> Result<UserProfileResponse, AppError>;
}
