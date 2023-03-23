// src/storage/engine/storage_engine.rs

use crate::database::record::Record;

pub trait StorageEngine {
    fn insert(&mut self, record: Record) -> Result<usize, String>;
    fn get(&self, record_id: usize) -> Option<&Record>;
    fn update(&mut self, record_id: usize, record: Record) -> Result<(), String>;
    fn delete(&mut self, record_id: usize) -> Result<(), String>;
}
