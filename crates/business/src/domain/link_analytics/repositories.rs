use crate::domain::link_analytics::{entities::LinkAnalytics, error::AnalyticsDomainError};

#[async_trait::async_trait]
pub trait AnalyticsRepository: Send + Sync {
    async fn save_batch(&self, items: Vec<LinkAnalytics>) -> Result<(), AnalyticsDomainError>;
}
