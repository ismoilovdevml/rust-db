// src/storage/engine/row_based.rs

use crate::database::record::Record;
use crate::storage::engine::storage_engine::StorageEngine;

pub struct RowBasedStorage {
    records: Vec<Record>,
}

impl RowBasedStorage {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }
}

impl StorageEngine for RowBasedStorage {
    fn insert(&mut self, record: Record) -> Result<usize, String> {
        let record_id = self.records.len();
        self.records.push(record);
        Ok(record_id)
    }

    fn get(&self, record_id: usize) -> Option<&Record> {
        self.records.get(record_id)
    }

    fn update(&mut self, record_id: usize, record: Record) -> Result<(), String> {
        if record_id < self.records.len() {
            self.records[record_id] = record;
            Ok(())
        } else {
            Err("Record ID not found.".to_string())
        }
    }

    fn delete(&mut self, record_id: usize) -> Result<(), String> {
        if record_id < self.records.len() {
            self.records.remove(record_id);
            Ok(())
        } else {
            Err("Record ID not found.".to_string())
        }
    }
}
