use std::net::IpAddr;

use business::domain::link_analytics::{
    services::GeoLookupService, value_objects::geo_data::GeoData,
};

pub struct MockGeoService;

#[async_trait::async_trait]
impl GeoLookupService for MockGeoService {
    async fn lookup(&self, ip: IpAddr) -> Result<GeoData, String> {
        let mock_pool = [
            ("VN", "Ho Chi Minh City"),
            ("VN", "Hanoi"),
            ("VN", "Da Nang"),
            ("US", "New York"),
            ("JP", "Tokyo"),
            ("UK", "London"),
            ("Unknown", "Unknown"),
        ];

        let index = match ip {
            IpAddr::V4(v4) => v4.octets()[3] as usize % mock_pool.len(),
            IpAddr::V6(v6) => v6.octets()[15] as usize % mock_pool.len(),
        };

        let (country, city) = mock_pool[index];

        Ok(GeoData::new(country.to_string(), city.to_string()))
    }

    async fn lookup_bulk(&self, ips: Vec<IpAddr>) -> Result<Vec<GeoData>, String> {
        let mut results = Vec::with_capacity(ips.len());

        for ip in ips {
            results.push(self.lookup(ip).await?);
        }

        Ok(results)
    }
}
