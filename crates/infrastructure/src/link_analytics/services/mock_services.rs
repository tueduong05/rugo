use std::net::IpAddr;

use business::domain::link_analytics::{
    entities::AnalyticsEvent,
    error::AnalyticsDomainError,
    services::{AnalyticsQueue, GeoLookupService, UserAgentParser},
    value_objects::{geo_data::GeoData, user_agent::UserAgent},
};

pub struct MockGeoService;

impl GeoLookupService for MockGeoService {
    fn lookup(&self, _ip: IpAddr) -> GeoData {
        GeoData {
            country_code: "VN".to_string(),
            city: "Ho Chi Minh City".to_string(),
        }
    }
}

pub struct MockUserAgentParser;

impl UserAgentParser for MockUserAgentParser {
    fn parse(&self, _ua_string: &Option<String>) -> UserAgent {
        UserAgent {
            browser: "Unknown".to_string(),
            os: "Unknown".to_string(),
            device: "Unknown".to_string(),
        }
    }
}

pub struct MockAnalyticsQueue;

#[async_trait::async_trait]
impl AnalyticsQueue for MockAnalyticsQueue {
    async fn push(&self, _event: AnalyticsEvent) -> Result<(), AnalyticsDomainError> {
        Ok(())
    }
}
