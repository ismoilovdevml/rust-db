// src/database/record.rs

#[derive(Debug, Clone)]
pub enum FieldValue {
    Null,
    Integer(i64),
    Float(f64),
    Text(String),
    // Add other field types as needed
}

#[derive(Debug, Clone)]
pub struct Record {
    pub fields: Vec<FieldValue>,
}

impl Record {
    pub fn new(fields: Vec<FieldValue>) -> Self {
        Self { fields }
    }
}
