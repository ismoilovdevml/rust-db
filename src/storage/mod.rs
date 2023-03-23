// src/storage/mod.rs

pub mod engine;

// Re-export StorageEngine trait for easier access
pub use engine::StorageEngine;
