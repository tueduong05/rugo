use business::domain::{common::error::BaseDomainError, link::error::LinkDomainError};
use redis::{Client, aio::ConnectionManager};

pub struct RedisLinkRepository {
    manager: ConnectionManager,
}

impl RedisLinkRepository {
    pub async fn new(client: Client) -> Result<Self, LinkDomainError> {
        let manager = client
            .get_connection_manager()
            .await
            .map_err(|e| BaseDomainError::Infrastructure(e.to_string()))?;

        Ok(Self { manager })
    }

    fn short_code_key(code: &str) -> String {
        format!("link:sc:{}", code)
    }
    fn click_counter_key(id: u64) -> String {
        format!("link:clicks:{}", id)
    }
    fn sync_set_key() -> &'static str {
        "links_pending_sync"
    }
}
