use business::domain::{
    common::error::BaseDomainError,
    link_analytics::{
        entities::LinkAnalytics,
        error::AnalyticsDomainError,
        repositories::AnalyticsRepository,
        value_objects::{
            analytics_dimension::AnalyticsDimension, stat_item::StatItem,
            time_series_point::TimeSeriesPoint,
        },
    },
};
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::link_analytics::persistence::models::LinkAnalyticsRecord;

pub struct PostgresAnalyticsRepository {
    pool: PgPool,
}

impl PostgresAnalyticsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AnalyticsRepository for PostgresAnalyticsRepository {
    #[tracing::instrument(skip(self, items), err, target = "infrastructure")]
    async fn save_batch(&self, items: Vec<LinkAnalytics>) -> Result<(), AnalyticsDomainError> {
        let records: Vec<LinkAnalyticsRecord> =
            items.iter().map(LinkAnalyticsRecord::from).collect();

        let link_ids: Vec<i64> = records.iter().map(|r| r.link_id).collect();
        let referrers: Vec<Option<String>> = records.iter().map(|r| r.referrer.clone()).collect();
        let uas: Vec<Option<String>> = records.iter().map(|r| r.user_agent.clone()).collect();

        let ua_infos: Vec<serde_json::Value> = records
            .iter()
            .map(|r| serde_json::to_value(&r.ua_info))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                BaseDomainError::Infrastructure(format!("UA serialization failed: {}", e))
            })?;

        let geos: Vec<serde_json::Value> = records
            .iter()
            .map(|r| serde_json::to_value(&r.geo))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                BaseDomainError::Infrastructure(format!("Geo serialization failed: {}", e))
            })?;

        let ips: Vec<String> = records.iter().map(|r| r.masked_ip.clone()).collect();
        let timestamps: Vec<DateTime<Utc>> = records.iter().map(|r| r.clicked_at).collect();

        sqlx::query!(
            r#"
            INSERT INTO link_analytics (link_id, referrer, user_agent, ua_info, geo, masked_ip, clicked_at)
            SELECT * FROM UNNEST($1::BIGINT[], $2::TEXT[], $3::TEXT[], $4::JSONB[], $5::JSONB[], $6::TEXT[], $7::TIMESTAMPTZ[])
            "#,
            &link_ids[..],
            &referrers as &[Option<String>],
            &uas as &[Option<String>],
            &ua_infos[..],
            &geos[..],
            &ips[..],
            &timestamps[..],
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }

    #[tracing::instrument(skip(self), err, target = "infrastructure")]
    async fn get_total_clicks(&self, link_id: u64) -> Result<u64, AnalyticsDomainError> {
        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM link_analytics WHERE link_id = $1",
            link_id as i64
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?
        .unwrap_or(0);

        Ok(count as u64)
    }

    #[tracing::instrument(skip(self), err, target = "infrastructure")]
    async fn get_daily_clicks(
        &self,
        link_id: u64,
        days: u32,
    ) -> Result<Vec<TimeSeriesPoint>, AnalyticsDomainError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                clicked_at::date as "date!", 
                COUNT(*)::bigint as "count!"
            FROM link_analytics
            WHERE link_id = $1 AND clicked_at >= CURRENT_DATE - ($2 * INTERVAL '1 day')
            GROUP BY clicked_at::date
            ORDER BY clicked_at::date ASC
            "#,
            link_id as i64,
            days as i32
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        let points = rows
            .into_iter()
            .map(|row| TimeSeriesPoint {
                date: row.date,
                count: row.count as u64,
            })
            .collect();

        Ok(points)
    }

    #[tracing::instrument(skip(self), err, target = "infrastructure")]
    async fn get_stats_by_dimension(
        &self,
        link_id: u64,
        dimension: AnalyticsDimension,
    ) -> Result<Vec<StatItem>, AnalyticsDomainError> {
        let link_id_i64 = link_id as i64;

        // Violate DRY to ensure compile-time safety
        let items: Vec<StatItem> = match dimension {
            AnalyticsDimension::Country => {
                sqlx::query!(
                    r#"SELECT COALESCE(geo->>'country_code', 'Unknown') as "label!", COUNT(*)::bigint as "count!" 
                       FROM link_analytics WHERE link_id = $1 GROUP BY 1 ORDER BY 2 DESC"#,
                    link_id_i64
                ).fetch_all(&self.pool).await.map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?
                .into_iter().map(|r| StatItem { label: r.label, count: r.count as u64 }).collect()
            },

            AnalyticsDimension::Browser => {
                sqlx::query!(
                    r#"SELECT COALESCE(ua_info->>'browser', 'Unknown') as "label!", COUNT(*)::bigint as "count!" 
                       FROM link_analytics WHERE link_id = $1 GROUP BY 1 ORDER BY 2 DESC"#,
                    link_id_i64
                ).fetch_all(&self.pool).await.map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?
                .into_iter().map(|r| StatItem { label: r.label, count: r.count as u64 }).collect()
            },

            AnalyticsDimension::Device => {
                sqlx::query!(
                    r#"SELECT COALESCE(ua_info->>'device', 'Unknown') as "label!", COUNT(*)::bigint as "count!" 
                       FROM link_analytics WHERE link_id = $1 GROUP BY 1 ORDER BY 2 DESC"#,
                    link_id_i64
                ).fetch_all(&self.pool).await.map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?
                .into_iter().map(|r| StatItem { label: r.label, count: r.count as u64 }).collect()
            },

            AnalyticsDimension::Referrer => {
                sqlx::query!(
                    r#"SELECT COALESCE(referrer, 'Unknown') as "label!", COUNT(*)::bigint as "count!" 
                       FROM link_analytics WHERE link_id = $1 GROUP BY 1 ORDER BY 2 DESC"#,
                    link_id_i64
                ).fetch_all(&self.pool).await.map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?
                .into_iter().map(|r| StatItem { label: r.label, count: r.count as u64 }).collect()
            },
        };

        Ok(items)
    }
}
