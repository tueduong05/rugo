use crate::{application::{error::AppError, link::use_cases::get_user_links::response::GetUserLinksResponse}, domain::user::value_objects::user_id::UserId};

pub mod response;
pub mod interactor;

#[async_trait::async_trait]
pub trait GetUserLinksUseCase: Send + Sync {
    async fn execute(&self, user_id: UserId) -> Result<GetUserLinksResponse, AppError>;
}
