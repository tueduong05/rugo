use std::mem;

use business::{
    application::link_analytics::workers::AnalyticsBatchWorker,
    domain::link_analytics::{
        entities::AnalyticsEvent,
        services::{GeoLookupService, UserAgentParser},
    },
};
use tokio::{
    sync::mpsc,
    time::{Duration, Instant, sleep},
};

pub async fn run_analytics_worker<G, U>(
    mut receiver: mpsc::Receiver<AnalyticsEvent>,
    worker: AnalyticsBatchWorker<G, U>,
) where
    G: GeoLookupService,
    U: UserAgentParser,
{
    let mut buffer = Vec::with_capacity(100);
    let batch_size = 100;
    let wait = Duration::from_secs(5);

    let mut last_flush = Instant::now();

    loop {
        let time_since_last_flush = last_flush.elapsed();
        let sleep_duration = wait.saturating_sub(time_since_last_flush);

        tokio::select! {
            event_opt = receiver.recv() => {
                match event_opt {
                    Some(event) => {
                        buffer.push(event);
                        if buffer.len() >= batch_size {
                            let batch = mem::take(&mut buffer);
                            let _ = worker.handle_batch(batch).await;
                            last_flush = Instant::now();
                        }
                    }

                    None => {
                        if !buffer.is_empty() {
                            let _ = worker.handle_batch(buffer).await;
                        }
                        break;
                    }
                }
            }

            _ = sleep(sleep_duration), if !buffer.is_empty() => {
                let batch = mem::take(&mut buffer);
                let _ = worker.handle_batch(batch).await;
                last_flush = Instant::now();
            }
        }
    }
}
