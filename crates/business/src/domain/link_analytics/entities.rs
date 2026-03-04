use chrono::{DateTime, Utc};

use crate::domain::link_analytics::value_objects::{geo_data::GeoData, user_agent::UserAgent};

pub struct LinkAnalytics {
    pub id: Option<u64>,
    pub link_id: u64,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub ua_info: UserAgent,
    pub geo: GeoData,
    pub masked_ip: Option<String>,
    pub clicked_at: DateTime<Utc>,
}
