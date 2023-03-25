// src/storage/transaction/transaction.rs

use std::collections::HashMap;
use crate::database::record::Record;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operation {
    Insert(usize, Record),
    Update(usize, Record, Record),
    Delete(usize, Record),
}

pub struct Transaction {
    id: Uuid,
    operations: Vec<Operation>,
    record_map: HashMap<usize, Record>,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            operations: Vec::new(),
            record_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, record: Record) -> usize {
        let record_id = self.record_map.len();
        self.record_map.insert(record_id, record.clone());
        self.operations.push(Operation::Insert(record_id, record));
        record_id
    }

    pub fn update(&mut self, record_id: usize, record: Record) -> Result<(), String> {
        if let Some(existing_record) = self.record_map.get(&record_id) {
            let old_record = existing_record.clone();
            self.record_map.insert(record_id, record.clone());
            self.operations.push(Operation::Update(record_id, old_record, record));
            Ok(())
        } else {
            Err("Record ID not found.".to_string())
        }
    }

    pub fn delete(&mut self, record_id: usize) -> Result<(), String> {
        if let Some(record) = self.record_map.remove(&record_id) {
            self.operations.push(Operation::Delete(record_id, record));
            Ok(())
        } else {
            Err("Record ID not found.".to_string())
        }
    }

    pub fn get_operations(&self) -> &Vec<Operation> {
        &self.operations
    }
}
