use std::net::IpAddr;

use chrono::{DateTime, Utc};

pub struct AnalyticsEvent {
    pub link_id: u64,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub ip: IpAddr,
    pub timestamp: DateTime<Utc>,
}
