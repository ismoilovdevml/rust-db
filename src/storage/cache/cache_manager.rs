// src/storage/cache/cache_manager.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::storage::cache::cache_entry::CacheEntry;

pub struct CacheManager {
    cache: Arc<Mutex<HashMap<usize, CacheEntry>>>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: usize) -> Option<CacheEntry> {
        self.cache.lock().unwrap().get(&key).cloned()
    }

    pub fn put(&self, key: usize, value: CacheEntry) {
        self.cache.lock().unwrap().insert(key, value);
    }

    pub fn remove(&self, key: usize) {
        self.cache.lock().unwrap().remove(&key);
    }

    pub fn clear(&self) {
        self.cache.lock().unwrap().clear();
    }
}
