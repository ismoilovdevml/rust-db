// src/database/index.rs

use crate::database::record::FieldValue;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Index {
    field_index: usize,
    index_map: BTreeMap<FieldValue, Vec<usize>>,
}

impl Index {
    pub fn new(field_index: usize) -> Self {
        Self {
            field_index,
            index_map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, record_id: usize, field_value: &FieldValue) {
        let entries = self.index_map.entry(field_value.clone()).or_insert_with(Vec::new);
        entries.push(record_id);
    }

    pub fn delete(&mut self, record_id: usize, field_value: &FieldValue) {
        if let Some(entries) = self.index_map.get_mut(field_value) {
            if let Some(pos) = entries.iter().position(|&id| id == record_id) {
                entries.remove(pos);
            }
        }
    }

    pub fn search(&self, field_value: &FieldValue) -> Option<&Vec<usize>> {
        self.index_map.get(field_value)
    }
}
