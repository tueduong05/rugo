use std::net::IpAddr;

use crate::domain::link_analytics::{
    entities::AnalyticsEvent,
    error::AnalyticsDomainError,
    value_objects::{geo_data::GeoData, user_agent::UserAgent},
};

pub trait UserAgentParser: Send + Sync {
    fn parse(&self, ua_string: &Option<String>) -> UserAgent;
}

pub trait GeoLookupService: Send + Sync {
    fn lookup(&self, ip: IpAddr) -> GeoData;
}

#[async_trait::async_trait]
pub trait AnalyticsQueue: Send + Sync {
    async fn push(&self, event: AnalyticsEvent) -> Result<(), AnalyticsDomainError>;
}
