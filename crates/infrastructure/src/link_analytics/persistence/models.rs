use business::domain::link_analytics::entities::LinkAnalytics;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize)]
pub struct UserAgentRecord {
    pub device: String,
    pub browser: String,
    pub os: String,
}

#[derive(Serialize, Deserialize)]
pub struct GeoDataRecord {
    pub country_code: String,
    pub city: String,
}

#[derive(FromRow)]
pub struct LinkAnalyticsRecord {
    pub link_id: i64,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    #[sqlx(json)]
    pub ua_info: UserAgentRecord,
    #[sqlx(json)]
    pub geo: GeoDataRecord,
    pub masked_ip: String,
    pub clicked_at: DateTime<Utc>,
}

impl From<&LinkAnalytics> for LinkAnalyticsRecord {
    fn from(analytics: &LinkAnalytics) -> Self {
        Self {
            link_id: analytics.link_id as i64,
            referrer: analytics.referrer.clone(),
            user_agent: analytics.user_agent.clone(),
            ua_info: UserAgentRecord {
                device: analytics.ua_info.device.clone(),
                browser: analytics.ua_info.browser.clone(),
                os: analytics.ua_info.os.clone(),
            },
            geo: GeoDataRecord {
                country_code: analytics.geo.country_code.clone(),
                city: analytics.geo.city.clone(),
            },
            masked_ip: analytics.masked_ip.clone(),
            clicked_at: analytics.clicked_at,
        }
    }
}
