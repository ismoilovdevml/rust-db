// src/storage/transaction/lock_manager.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub enum LockMode {
    Shared,
    Exclusive,
}

pub struct LockManager {
    lock_table: Arc<Mutex<HashMap<usize, (Uuid, LockMode)>>>,
}

impl LockManager {
    pub fn new() -> Self {
        Self {
            lock_table: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn acquire_lock(&self, transaction_id: Uuid, record_id: usize, mode: LockMode) -> Result<(), String> {
        let mut lock_table = self.lock_table.lock().unwrap();

        if let Some((existing_transaction_id, existing_mode)) = lock_table.get(&record_id) {
            if *existing_transaction_id == transaction_id {
                return Ok(());
            }

            if *existing_mode == LockMode::Exclusive || mode == LockMode::Exclusive {
                return Err("Cannot acquire lock, resource is locked by another transaction.".to_string());
            }
        }

        lock_table.insert(record_id, (transaction_id, mode));
        Ok(())
    }

    pub fn release_lock(&self, transaction_id: Uuid, record_id: usize) -> Result<(), String> {
        let mut lock_table = self.lock_table.lock().unwrap();

        if let Some((existing_transaction_id, _)) = lock_table.get(&record_id) {
            if *existing_transaction_id == transaction_id {
                lock_table.remove(&record_id);
                return Ok(());
            }
        }

        Err("Cannot release lock, not owned by the transaction.".to_string())
    }
}
