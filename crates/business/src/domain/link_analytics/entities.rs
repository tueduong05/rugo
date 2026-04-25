use std::net::IpAddr;

use chrono::{DateTime, Utc};

use crate::domain::link_analytics::value_objects::{geo_data::GeoData, user_agent::UserAgent};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_analytics_valid() {
        let analytics = LinkAnalytics::new(
            1,
            Some("https://example.com".to_string()),
            Some("Mozilla/5.0".to_string()),
            UserAgent::new(
                "Chrome".to_string(),
                "Windows".to_string(),
                "Desktop".to_string(),
            ),
            GeoData::new("VN".to_string(), "Hanoi".to_string()),
            "203.0.113.42".parse().expect("valid ip"),
            Utc::now(),
        );

        assert_eq!(analytics.link_id, 1);
        assert_eq!(analytics.masked_ip, "203.0.113.0");
    }

    #[test]
    fn test_mask_ip_ipv4() {
        assert_eq!(
            LinkAnalytics::mask_ip("203.0.113.42".parse().expect("valid ip")),
            "203.0.113.0"
        );
    }

    #[test]
    fn test_mask_ip_ipv6() {
        assert_eq!(
            LinkAnalytics::mask_ip("2001:db8:abcd:12::1".parse().expect("valid ip")),
            "2001:db8:abcd:12:0:0:0:0"
        );
    }
}
