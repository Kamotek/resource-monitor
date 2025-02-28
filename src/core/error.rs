// core/error.rs
pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum CollectorError {
    #[error("Failed to execute collection task")]
    TaskFailed,

    #[error("Invalid interval: {0}")]
    InvalidInterval(u64),
}