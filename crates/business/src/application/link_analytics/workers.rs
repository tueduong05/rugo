use crate::{
    application::error::AppError,
    domain::link_analytics::{
        entities::{AnalyticsEvent, LinkAnalytics},
        repositories::AnalyticsRepository,
        services::{GeoLookupService, UserAgentParser},
    },
};

// TODO: Add worker runner

pub struct AnalyticsBatchWorker<R, G, U> {
    analytics_repo: R,
    geo_provider: G,
    ua_parser: U,
}

impl<R, G, U> AnalyticsBatchWorker<R, G, U>
where
    R: AnalyticsRepository,
    G: GeoLookupService,
    U: UserAgentParser,
{
    pub fn new(analytics_repo: R, geo_provider: G, ua_parser: U) -> Self {
        Self {
            analytics_repo,
            geo_provider,
            ua_parser,
        }
    }

    pub async fn handle_batch(&self, events: Vec<AnalyticsEvent>) -> Result<(), AppError> {
        let mut processed = Vec::with_capacity(events.len());

        for event in events {
            let ua_info = self.ua_parser.parse(&event.user_agent);
            let geo = self.geo_provider.lookup(event.ip);

            let link_analytics = LinkAnalytics::new(
                event.link_id,
                event.referrer,
                event.user_agent,
                ua_info,
                geo,
                event.ip,
                event.timestamp,
            );

            processed.push(link_analytics);
        }

        self.analytics_repo.save_batch(processed).await?;

        Ok(())
    }

    pub async fn flush(&self, events: Vec<AnalyticsEvent>) -> Result<(), AppError> {
        if events.is_empty() {
            return Ok(());
        }

        self.handle_batch(events).await
    }
}
