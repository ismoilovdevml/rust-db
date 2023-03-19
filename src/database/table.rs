// src/database/table.rs

use crate::database::{record::Record, schema::Schema};
use std::collections::HashMap;

pub struct Table {
    schema: Schema,
    records: HashMap<usize, Record>,
    next_record_id: usize,
}

impl Table {
    pub fn new(schema: Schema) -> Self {
        Self {
            schema,
            records: HashMap::new(),
            next_record_id: 0,
        }
    }

    pub fn insert_record(&mut self, record: Record) -> Result<usize, String> {
        if record.fields.len() != self.schema.fields.len() {
            return Err(format!(
                "Record has {} fields, expected {} fields.",
                record.fields.len(),
                self.schema.fields.len()
            ));
        }

        let record_id = self.next_record_id;
        self.records.insert(record_id, record);
        self.next_record_id += 1;
        Ok(record_id)
    }

    pub fn get_record(&self, record_id: usize) -> Option<&Record> {
        self.records.get(&record_id)
    }

    pub fn update_record(&mut self, record_id: usize, record: Record) -> Result<(), String> {
        if record.fields.len() != self.schema.fields.len() {
            return Err(format!(
                "Record has {} fields, expected {} fields.",
                record.fields.len(),
                self.schema.fields.len()
            ));
        }

        if let Some(existing_record) = self.records.get_mut(&record_id) {
            *existing_record = record;
            Ok(())
        } else {
            Err(format!("Record with ID {} not found.", record_id))
        }
    }

    pub fn delete_record(&mut self, record_id: usize) -> Result<(), String> {
        if let Some(_) = self.records.remove(&record_id) {
            Ok(())
        } else {
            Err(format!("Record with ID {} not found.", record_id))
        }
    }
}
