use async_trait::async_trait;
// adapters/monitor/sysinfo_collector.rs
use sysinfo::{CpuExt, System, SystemExt};
use tokio::task;
use crate::core::{
    ports::MetricsCollector,
    model::Metric,
    error::CollectorError
};

pub struct SysInfoCollector {
    interval: std::sync::Arc<tokio::sync::Mutex<u64>>,
}

impl SysInfoCollector {
    pub fn new(initial_interval: u64) -> Self {
        Self {
            interval: std::sync::Arc::new(tokio::sync::Mutex::new(initial_interval)),
        }
    }
}

#[async_trait]
impl MetricsCollector for SysInfoCollector {
    async fn collect(&self) -> Result<Metric, CollectorError> {

        task::spawn_blocking(move || {
            // Create new System instance inside the closure
            let mut sys = System::new_all();
            sys.refresh_all();

            Metric {
                id: 0,
                cpu_usage: sys.global_cpu_info().cpu_usage(),
                memory_used: sys.used_memory(),
                timestamp: chrono::Utc::now(),
            }
        })
            .await
            .map_err(|_| CollectorError::TaskFailed)
    }

    fn set_interval(&mut self, interval_secs: u64) {
        let interval = self.interval.clone();
        tokio::spawn(async move {
            *interval.lock().await = interval_secs;
        });
    }
}