use business::domain::common::{
    events::analytics_event::AnalyticsEvent, services::analytics_queue::AnalyticsQueue,
};
use tokio::sync::mpsc;

// NOTE: Use a better queue and outbox to avoid data lost when crashes
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
    async fn push(&self, event: AnalyticsEvent) -> Result<(), String> {
        self.sender.send(event).await.map_err(|s| s.to_string())?;

        Ok(())
    }
}
