// src/database/mod.rs

pub mod database;
pub mod table;
pub mod record;
pub mod schema;
pub mod index;
pub mod query;

// Re-export components for easier access
pub use database::Database;
pub use table::Table;
pub use record::Record;
pub use schema::Schema;
pub use index::Index;
pub use query::{parser::Parser, planner::Planner, executor::Executor};
