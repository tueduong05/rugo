pub struct GeoData {
    pub country_code: String,
    pub city: String,
}

impl GeoData {
    pub fn new(country_code: String, city: String) -> Self {
        Self { country_code, city }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_data_new() {
        let geo = GeoData::new("VN".to_string(), "Hanoi".to_string());

        assert_eq!(geo.country_code, "VN");
        assert_eq!(geo.city, "Hanoi");
    }
}
