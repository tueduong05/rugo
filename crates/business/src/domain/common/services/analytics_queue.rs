use crate::domain::common::events::analytics_event::AnalyticsEvent;

#[async_trait::async_trait]
pub trait AnalyticsQueue: Send + Sync {
    async fn push(&self, event: AnalyticsEvent) -> Result<(), String>;
}
