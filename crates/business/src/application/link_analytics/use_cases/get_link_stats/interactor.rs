use std::sync::Arc;

use chrono::Utc;

use crate::{
    application::{
        error::AppError,
        link_analytics::use_cases::get_link_stats::{
            GetLinkStatsUseCase,
            response::{GetLinkStatsResponse, StatItemDTO, TimeSeriesPointDTO},
        },
    },
    domain::{
        common::error::BaseDomainError,
        link::{error::LinkDomainError, repositories::LinkRepository},
        link_analytics::{
            repositories::AnalyticsRepository,
            value_objects::analytics_dimension::AnalyticsDimension,
        },
        user::{error::UserDomainError, value_objects::user_id::UserId},
    },
};

pub struct GetLinkStatsInteractor {
    link_repo: Arc<dyn LinkRepository>,
    analytics_repo: Arc<dyn AnalyticsRepository>,
}

impl GetLinkStatsInteractor {
    pub fn new(
        link_repo: Arc<dyn LinkRepository>,
        analytics_repo: Arc<dyn AnalyticsRepository>,
    ) -> Self {
        Self {
            link_repo,
            analytics_repo,
        }
    }
}

#[async_trait::async_trait]
impl GetLinkStatsUseCase for GetLinkStatsInteractor {
    async fn execute(
        &self,
        user_id: UserId,
        link_id: u64,
    ) -> Result<GetLinkStatsResponse, AppError> {
        let link = self.link_repo.find_by_id(link_id).await?.ok_or_else(|| {
            LinkDomainError::from(BaseDomainError::ResourceNotFound("Link".into()))
        })?;

        if link.user_id != Some(user_id) {
            return Err(UserDomainError::AccessDenied.into());
        }

        let total_clicks = self.analytics_repo.get_total_clicks(link_id).await?;
        let countries = self
            .analytics_repo
            .get_stats_by_dimension(link_id, AnalyticsDimension::Country)
            .await?;
        let browsers = self
            .analytics_repo
            .get_stats_by_dimension(link_id, AnalyticsDimension::Browser)
            .await?;
        let devices = self
            .analytics_repo
            .get_stats_by_dimension(link_id, AnalyticsDimension::Device)
            .await?;
        let daily_data = self.analytics_repo.get_daily_clicks(link_id, 30).await?;

        Ok(GetLinkStatsResponse {
            original_link: link.original_link.to_string(),
            total_clicks,
            countries: StatItemDTO::from_domain_vec(countries),
            browsers: StatItemDTO::from_domain_vec(browsers),
            devices: StatItemDTO::from_domain_vec(devices),
            daily_clicks: daily_data
                .into_iter()
                .map(TimeSeriesPointDTO::from)
                .collect(),
            last_updated: Utc::now(),
        })
    }
}
