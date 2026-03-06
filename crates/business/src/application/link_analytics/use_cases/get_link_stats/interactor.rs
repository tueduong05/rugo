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
        common::{services::link_provider::LinkProvider, value_objects::user_id::UserId},
        link_analytics::{
            repositories::AnalyticsRepository,
            value_objects::analytics_dimension::AnalyticsDimension,
        },
    },
};

pub struct GetLinkStatsInteractor {
    link_provider: Arc<dyn LinkProvider>,
    analytics_repo: Arc<dyn AnalyticsRepository>,
}

impl GetLinkStatsInteractor {
    pub fn new(
        link_provider: Arc<dyn LinkProvider>,
        analytics_repo: Arc<dyn AnalyticsRepository>,
    ) -> Self {
        Self {
            link_provider,
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
        let original_link = self
            .link_provider
            .verify_ownership(link_id, user_id)
            .await?;

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
            original_link: original_link.to_string(),
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
