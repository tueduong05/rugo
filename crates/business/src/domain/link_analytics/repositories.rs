use crate::domain::link_analytics::{entities::LinkAnalytics, error::AnalyticsDomainError};

pub trait AnalyticsRepository: Send + Sync {
    fn save_batch(&self, items: Vec<LinkAnalytics>) -> Result<(), AnalyticsDomainError>;
}
