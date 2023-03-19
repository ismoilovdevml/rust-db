// src/lib.rs

pub mod config;
pub mod database;
pub mod storage;
pub mod transaction;
pub mod logging;
pub mod cache;
pub mod api_server;
pub mod cli;

// You can also re-export key components if needed
pub use config::settings::Settings;
pub use database::{database::Database, table::Table, record::Record, schema::Schema, index::Index};
pub use storage::{
    engine::{StorageEngine, RowBased, Columnar},
    file_manager::FileManager,
    file_operations::FileOperations,
};
pub use transaction::{transaction::Transaction, transaction_manager::TransactionManager, lock_manager::LockManager};
pub use logging::logger::Logger;
pub use cache::cache_manager::CacheManager;
pub use api_server::{server::Server, router::Router, handler::Handler};
pub use cli::{commands::Commands, parser::CliParser};
