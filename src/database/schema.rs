// src/database/schema.rs

use crate::database::record::FieldValue;

#[derive(Debug, Clone)]
pub enum FieldType {
    Integer,
    Float,
    Text,
    // Add other field types as needed
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
}

impl Field {
    pub fn new(name: String, field_type: FieldType) -> Self {
        Self { name, field_type }
    }
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub fields: Vec<Field>,
}

impl Schema {
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    pub fn validate_record(&self, record: &FieldValue) -> Result<(), String> {
        // Validate the record based on the schema
        // ...
    }
}
