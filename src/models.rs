use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlModel {
    pub id: i32,
    pub url: String,
    pub short_code: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub access_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlResponse {
    pub id: i32,
    pub url: String,
    pub short_code: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlStatsResponse {
    pub id: i32,
    pub url: String,
    pub short_code: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub access_count: i32,
}

impl From<UrlModel> for UrlResponse {
    fn from(m: UrlModel) -> Self {
        Self {
            id: m.id,
            url: m.url,
            short_code: m.short_code,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

impl From<UrlModel> for UrlStatsResponse {
    fn from(m: UrlModel) -> Self {
        Self {
            id: m.id,
            url: m.url,
            short_code: m.short_code,
            created_at: m.created_at,
            updated_at: m.updated_at,
            access_count: m.access_count,
        }
    }
}
