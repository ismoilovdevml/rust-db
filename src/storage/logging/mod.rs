// src/storage/logging/mod.rs

pub mod log_manager;
pub mod log_entry;

// Re-export components for easier access
pub use log_manager::LogManager;
pub use log_entry::LogEntry;
