use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;

use crate::domain::link_analytics::value_objects::{
    stat_item::StatItem, time_series_point::TimeSeriesPoint,
};

#[derive(Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct StatItemDTO {
    pub label: String,
    pub count: u64,
    pub percentage: f64,
}

impl StatItemDTO {
    pub fn from_domain_vec(items: Vec<StatItem>) -> Vec<Self> {
        let total: u64 = items.iter().map(|i| i.count).sum();

        items
            .into_iter()
            .map(|item| Self {
                label: item.label,
                count: item.count,
                percentage: if total > 0 {
                    (item.count as f64 / total as f64) * 100.0
                } else {
                    0.0
                },
            })
            .collect()
    }
}

#[derive(Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct TimeSeriesPointDTO {
    pub date: NaiveDate,
    pub count: u64,
}

impl From<TimeSeriesPoint> for TimeSeriesPointDTO {
    fn from(value: TimeSeriesPoint) -> Self {
        Self {
            date: value.date,
            count: value.count,
        }
    }
}

#[derive(Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct GetLinkStatsResponse {
    pub original_link: String,
    pub total_clicks: u64,
    pub countries: Vec<StatItemDTO>,
    pub browsers: Vec<StatItemDTO>,
    pub devices: Vec<StatItemDTO>,
    pub daily_clicks: Vec<TimeSeriesPointDTO>,
    pub last_updated: DateTime<Utc>,
}
