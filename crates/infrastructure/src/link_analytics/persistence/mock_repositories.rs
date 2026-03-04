use std::sync::Mutex;

use business::domain::link_analytics::{
    entities::LinkAnalytics,
    error::AnalyticsDomainError,
    repositories::AnalyticsRepository,
    value_objects::{
        analytics_dimension::AnalyticsDimension, stat_item::StatItem,
        time_series_point::TimeSeriesPoint,
    },
};
use chrono::{Duration, Utc};

pub struct MockAnalyticsRepository {
    pub events: Mutex<Vec<LinkAnalytics>>,
}

impl MockAnalyticsRepository {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait::async_trait]
impl AnalyticsRepository for MockAnalyticsRepository {
    async fn save_batch(&self, items: Vec<LinkAnalytics>) -> Result<(), AnalyticsDomainError> {
        let mut events = self.events.lock().unwrap();
        events.extend(items);
        Ok(())
    }

    async fn get_total_clicks(&self, _link_id: u64) -> Result<u64, AnalyticsDomainError> {
        Ok(1250)
    }

    async fn get_daily_clicks(
        &self,
        _link_id: u64,
        days: u32,
    ) -> Result<Vec<TimeSeriesPoint>, AnalyticsDomainError> {
        let now = Utc::now().date_naive();

        let points = (0..days)
            .rev()
            .map(|i| TimeSeriesPoint {
                date: now - Duration::days(i as i64),
                count: 10 + (i as u64 * 2),
            })
            .collect();

        Ok(points)
    }

    async fn get_stats_by_dimension(
        &self,
        _link_id: u64,
        dimension: AnalyticsDimension,
    ) -> Result<Vec<StatItem>, AnalyticsDomainError> {
        let data = match dimension {
            AnalyticsDimension::Country => vec![
                StatItem {
                    label: "US".into(),
                    count: 500,
                },
                StatItem {
                    label: "GB".into(),
                    count: 300,
                },
                StatItem {
                    label: "DE".into(),
                    count: 200,
                },
            ],
            AnalyticsDimension::Browser => vec![
                StatItem {
                    label: "Chrome".into(),
                    count: 800,
                },
                StatItem {
                    label: "Firefox".into(),
                    count: 300,
                },
                StatItem {
                    label: "Safari".into(),
                    count: 150,
                },
            ],
            AnalyticsDimension::Device => vec![
                StatItem {
                    label: "Mobile".into(),
                    count: 900,
                },
                StatItem {
                    label: "Desktop".into(),
                    count: 350,
                },
            ],
            AnalyticsDimension::Referrer => vec![
                StatItem {
                    label: "Twitter".into(),
                    count: 600,
                },
                StatItem {
                    label: "Direct".into(),
                    count: 650,
                },
            ],
        };

        Ok(data)
    }
}
