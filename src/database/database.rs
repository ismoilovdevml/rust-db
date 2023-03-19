// src/database/database.rs

use crate::storage::engine::StorageEngine;
use crate::transaction::transaction_manager::TransactionManager;
use crate::cache::cache_manager::CacheManager;
use std::sync::Arc;
use std::collections::HashMap;

pub struct Database {
    pub storage_engine: Arc<dyn StorageEngine>,
    pub transaction_manager: Arc<TransactionManager>,
    pub cache_manager: Arc<CacheManager>,
    tables: HashMap<String, Table>,
}

impl Database {
    pub fn new(
        storage_engine: Arc<dyn StorageEngine>,
        transaction_manager: Arc<TransactionManager>,
        cache_manager: Arc<CacheManager>,
    ) -> Self {
        Self {
            storage_engine,
            transaction_manager,
            cache_manager,
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, name: String, schema: Schema) -> Result<(), String> {
        if self.tables.contains_key(&name) {
            return Err(format!("Table {} already exists.", name));
        }

        let table = Table::new(schema);
        self.tables.insert(name, table);
        Ok(())
    }

    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }
}
