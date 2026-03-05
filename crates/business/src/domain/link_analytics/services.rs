use std::net::IpAddr;

use crate::domain::link_analytics::{
    entities::AnalyticsEvent,
    value_objects::{geo_data::GeoData, user_agent::UserAgent},
};

pub trait UserAgentParser: Send + Sync {
    fn parse(&self, ua_string: &Option<String>) -> UserAgent;
}

#[async_trait::async_trait]
pub trait GeoLookupService: Send + Sync {
    async fn lookup(&self, ip: IpAddr) -> Result<GeoData, String>;

    async fn lookup_bulk(&self, ips: Vec<IpAddr>) -> Result<Vec<GeoData>, String>;
}

#[async_trait::async_trait]
pub trait AnalyticsQueue: Send + Sync {
    async fn push(&self, event: AnalyticsEvent) -> Result<(), String>;
}
