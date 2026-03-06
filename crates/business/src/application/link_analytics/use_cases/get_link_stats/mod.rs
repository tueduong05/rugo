use crate::{
    application::{
        error::AppError, link_analytics::use_cases::get_link_stats::response::GetLinkStatsResponse,
    },
    domain::common::value_objects::user_id::UserId,
};

pub mod interactor;
pub mod response;

#[async_trait::async_trait]
pub trait GetLinkStatsUseCase: Send + Sync {
    async fn execute(
        &self,
        user_id: UserId,
        link_id: u64,
    ) -> Result<GetLinkStatsResponse, AppError>;
}
