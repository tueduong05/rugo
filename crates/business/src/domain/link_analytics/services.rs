use std::net::IpAddr;

use crate::domain::link_analytics::value_objects::{geo_data::GeoData, user_agent::UserAgent};

pub trait UserAgentParser: Send + Sync {
    fn parse(&self, ua_string: &Option<String>) -> UserAgent;
}

pub trait GeoLookupService: Send + Sync {
    fn lookup(&self, ip: IpAddr) -> GeoData;
}
