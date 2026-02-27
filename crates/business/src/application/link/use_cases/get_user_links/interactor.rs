use std::sync::Arc;

use crate::{
    application::{
        error::AppError,
        link::use_cases::get_user_links::{
            GetUserLinksUseCase,
            response::{GetUserLinkItem, GetUserLinksResponse},
        },
    },
    domain::{link::repositories::LinkRepository, user::value_objects::user_id::UserId},
};

pub struct GetUserLinksInteractor {
    link_repo: Arc<dyn LinkRepository>,
}

impl GetUserLinksInteractor {
    pub fn new(link_repo: Arc<dyn LinkRepository>) -> Self {
        Self { link_repo }
    }
}

#[async_trait::async_trait]
impl GetUserLinksUseCase for GetUserLinksInteractor {
    async fn execute(&self, user_id: UserId) -> Result<GetUserLinksResponse, AppError> {
        let links = self.link_repo.find_by_user_id(&user_id).await?;
        let total_count = links.len();

        let items: Vec<GetUserLinkItem> = links
            .into_iter()
            .map(|link| GetUserLinkItem {
                original_link: link.original_link.to_string(),
                short_code: link.short_code.to_string(),
                is_custom: link.is_custom,
                expires_at: link.expires_at,
                max_clicks: link.max_clicks,
                is_active: link.is_active,
                created_at: link.created_at,
                updated_at: link.updated_at,
            })
            .collect();

        Ok(GetUserLinksResponse {
            links: items,
            total_count,
        })
    }
}
