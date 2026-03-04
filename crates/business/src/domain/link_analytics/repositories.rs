use crate::domain::link_analytics::{
    entities::LinkAnalytics,
    error::AnalyticsDomainError,
    value_objects::{
        analytics_dimension::AnalyticsDimension, stat_item::StatItem,
        time_series_point::TimeSeriesPoint,
    },
};

#[async_trait::async_trait]
pub trait AnalyticsRepository: Send + Sync {
    async fn save_batch(&self, items: Vec<LinkAnalytics>) -> Result<(), AnalyticsDomainError>;

    async fn get_total_clicks(&self, link_id: u64) -> Result<u64, AnalyticsDomainError>;

    async fn get_daily_clicks(
        &self,
        link_id: u64,
        days: u32,
    ) -> Result<Vec<TimeSeriesPoint>, AnalyticsDomainError>;

    async fn get_stats_by_dimension(
        &self,
        link_id: u64,
        dimension: AnalyticsDimension,
    ) -> Result<Vec<StatItem>, AnalyticsDomainError>;
}
