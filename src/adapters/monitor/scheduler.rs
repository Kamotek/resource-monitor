// adapters/monitor/scheduler.rs
use tokio::{sync::mpsc::Sender, time};
use crate::core::{ports::MetricsCollector, error::CollectorError, Metric};

pub struct MetricsScheduler<C: MetricsCollector> {
    collector: C,
    sender: Sender<Result<Metric, CollectorError>>,
}

impl<C: MetricsCollector> MetricsScheduler<C> {
    pub fn new(collector: C, sender: Sender<Result<Metric, CollectorError>>) -> Self {
        Self { collector, sender }
    }

    pub async fn run(self) {
        let mut interval = time::interval(time::Duration::from_secs(5));

        loop {
            interval.tick().await;
            let result = self.collector.collect().await;
            if let Err(e) = self.sender.send(result).await {
                log::error!("Failed to send metrics: {}", e);
                break;
            }
        }
    }
}