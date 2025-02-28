// core/model/metric.rs
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Metric {
    pub(crate) id: i32,  // Własny typ, nie prymityw!
    pub(crate) cpu_usage: f32, // Newtype pattern
    pub(crate) memory_used: u64, // tu bedzie trzeba najpewniej zmienic pozniej z u32 na Bajty
    pub(crate) timestamp: DateTime<Utc>,
}

// Newtype dla bezpieczeństwa typów
#[derive(Debug, Clone, Copy)]
pub struct CpuUsage(f32);
#[derive(Debug, Clone, Copy)]
pub struct MemoryUsage(f32);