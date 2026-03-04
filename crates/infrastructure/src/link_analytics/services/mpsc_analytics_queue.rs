use business::domain::{
    common::error::BaseDomainError,
    link_analytics::{
        entities::AnalyticsEvent, error::AnalyticsDomainError, services::AnalyticsQueue,
    },
};
use tokio::sync::mpsc;

pub struct MPSCAnalyticsQueue {
    sender: mpsc::Sender<AnalyticsEvent>,
}

impl MPSCAnalyticsQueue {
    pub fn new(sender: mpsc::Sender<AnalyticsEvent>) -> Self {
        Self { sender }
    }
}

#[async_trait::async_trait]
impl AnalyticsQueue for MPSCAnalyticsQueue {
    async fn push(&self, event: AnalyticsEvent) -> Result<(), AnalyticsDomainError> {
        self.sender
            .send(event)
            .await
            .map_err(|_| BaseDomainError::Infrastructure("Analytics channel closed".into()))?;

        Ok(())
    }
}
