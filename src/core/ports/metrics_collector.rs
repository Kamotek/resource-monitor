// core/ports/metrics_collector.rs
use async_trait::async_trait;
use crate::core::{model::Metric, error::CollectorError};

#[async_trait]
pub trait MetricsCollector: Send + Sync {
    async fn collect(&self) -> Result<Metric, CollectorError>;

    fn set_interval(&mut self, interval_secs: u64);
}