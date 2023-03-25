// src/storage/logging/logger.rs

use std::sync::{Arc, Mutex};
use crate::storage::logging::log_entry::LogEntry;

pub struct Logger {
    log_entries: Arc<Mutex<Vec<LogEntry>>>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            log_entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn log(&self, log_entry: LogEntry) {
        self.log_entries.lock().unwrap().push(log_entry);
    }

    pub fn get_logs(&self) -> Vec<LogEntry> {
        self.log_entries.lock().unwrap().clone()
    }
}
