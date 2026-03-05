use std::net::IpAddr;

use chrono::{DateTime, Utc};

use crate::domain::link_analytics::value_objects::{geo_data::GeoData, user_agent::UserAgent};

pub struct AnalyticsEvent {
    pub link_id: u64,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub ip: IpAddr,
    pub timestamp: DateTime<Utc>,
}

pub struct LinkAnalytics {
    pub link_id: u64,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub ua_info: UserAgent,
    pub geo: GeoData,
    pub masked_ip: String,
    pub clicked_at: DateTime<Utc>,
}

impl LinkAnalytics {
    pub fn new(
        link_id: u64,
        referrer: Option<String>,
        user_agent: Option<String>,
        ua_info: UserAgent,
        geo: GeoData,
        ip: IpAddr,
        clicked_at: DateTime<Utc>,
    ) -> Self {
        let masked_ip = Self::mask_ip(ip);

        Self {
            link_id,
            referrer,
            user_agent,
            ua_info,
            geo,
            masked_ip,
            clicked_at,
        }
    }

    fn mask_ip(ip: IpAddr) -> String {
        match ip {
            IpAddr::V4(v4) => {
                let octets = v4.octets();
                format!("{}.{}.{}.0", octets[0], octets[1], octets[2])
            }
            IpAddr::V6(v6) => {
                let segments = v6.segments();
                format!(
                    "{:x}:{:x}:{:x}:{:x}:0:0:0:0",
                    segments[0], segments[1], segments[2], segments[3]
                )
            }
        }
    }
}
