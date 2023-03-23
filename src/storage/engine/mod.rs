// src/storage/engine/mod.rs

pub mod in_memory;
pub mod on_disk;

// Re-export storage engines for easier access
pub use in_memory::InMemoryStorage;
pub use on_disk::OnDiskStorage;

use crate::database::record::Record;

pub trait StorageEngine {
    fn insert(&mut self, record: Record) -> Result<usize, String>;
    fn get(&self, record_id: usize) -> Option<&Record>;
    fn update(&mut self, record_id: usize, record: Record) -> Result<(), String>;
    fn delete(&mut self, record_id: usize) -> Result<(), String>;
}
