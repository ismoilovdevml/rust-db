// src/storage/cache/mod.rs

pub mod cache_manager;
pub mod cache_entry;

// Re-export components for easier access
pub use cache_manager::CacheManager;
pub use cache_entry::CacheEntry;
