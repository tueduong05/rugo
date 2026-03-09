use std::{net::IpAddr, sync::Arc};

use crate::domain::{
    common::events::analytics_event::AnalyticsEvent,
    link_analytics::{
        entities::LinkAnalytics,
        repositories::AnalyticsRepository,
        services::{GeoLookupService, UserAgentParser},
    },
};

pub struct AnalyticsBatchWorker<G, U> {
    analytics_repo: Arc<dyn AnalyticsRepository>,
    geo_provider: G,
    ua_parser: U,
}

impl<G, U> AnalyticsBatchWorker<G, U>
where
    G: GeoLookupService,
    U: UserAgentParser,
{
    pub fn new(
        analytics_repo: Arc<dyn AnalyticsRepository>,
        geo_provider: G,
        ua_parser: U,
    ) -> Self {
        Self {
            analytics_repo,
            geo_provider,
            ua_parser,
        }
    }

    pub async fn handle_batch(&self, events: Vec<AnalyticsEvent>) -> Result<(), String> {
        let ips: Vec<IpAddr> = events.iter().map(|e| e.ip).collect();

        let geo_results = self.geo_provider.lookup_bulk(ips).await?;

        let mut processed = Vec::with_capacity(events.len());

        for (event, geo) in events.into_iter().zip(geo_results.into_iter()) {
            let ua_info = self.ua_parser.parse(&event.user_agent);

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

        self.analytics_repo
            .save_batch(processed)
            .await
            .map_err(|s| s.to_string())?;

        Ok(())
    }
}
