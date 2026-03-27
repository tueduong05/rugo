use std::sync::Arc;

use business::domain::link::repositories::LinkRepository;
use chrono::Utc;
use tokio::time::{Duration, sleep};

use crate::link::persistence::redis_link_repository::RedisLinkRepository;

pub async fn run_link_clicks_sync_worker(
    redis_link_repo: Arc<RedisLinkRepository>,
    pg_link_repo: Arc<dyn LinkRepository>,
) {
    let wait = Duration::from_secs(5);

    loop {
        sleep(wait).await;

        let _ = redis_link_repo.requeue_inflight_clicks().await;

        let claimed_clicks = match redis_link_repo.claim_pending_clicks().await {
            Ok(claimed_clicks) => claimed_clicks,
            Err(_) => continue,
        };

        let now = Utc::now();

        for (link_id, count) in claimed_clicks {
            // NOTE: Increment might fail if link is invalid at the moment of flush
            // which the link cant be used anyways, but clicks count will be inconsistent
            // This is currently accepted, might need to mitigate later
            match pg_link_repo.increment_clicks(link_id, count, now).await {
                Ok(_) => {
                    let _ = redis_link_repo.ack_persisted_clicks(link_id, count).await;
                }
                Err(_) => {
                    let _ = redis_link_repo.release_claimed_click(link_id).await;
                }
            }
        }
    }
}
