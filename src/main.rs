mod adapters;
mod core;
mod shared;

// main.rs
use tokio::sync::mpsc;
use crate::adapters::monitor::SysInfoCollector;
use crate::adapters::monitor::MetricsScheduler;
use crate::core::ports::MetricsCollector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel(10);

    // Initialize collector with 5s interval
    let mut collector = SysInfoCollector::new(5);

    // Change interval dynamically
    collector.set_interval(10);

    // Start scheduler
    let scheduler = MetricsScheduler::new(collector, tx);
    tokio::spawn(async move {
        scheduler.run().await;
    });

    // Process metrics
    while let Some(metric_result) = rx.recv().await {
        match metric_result {
            Ok(metric) => println!("Collected: {:?}", metric),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}