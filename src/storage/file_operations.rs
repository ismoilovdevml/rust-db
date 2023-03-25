// src/storage/file_operations.rs

use std::path::Path;
use crate::storage::file_manager::FileManager;

pub struct FileOperations {
    file_manager: FileManager,
}

impl FileOperations {
    pub fn new(file_path: &Path) -> std::io::Result<Self> {
        let file_manager = FileManager::new(file_path)?;

        Ok(Self { file_manager })
    }

    pub fn read(&mut self, offset: u64, size: usize) -> std::io::Result<Vec<u8>> {
        self.file_manager.read(offset, size)
    }

    pub fn write(&mut self, offset: u64, data: &[u8]) -> std::io::Result<()> {
        self.file_manager.write(offset, data)
    }

    pub fn append(&mut self, data: &[u8]) -> std::io::Result<u64> {
        self.file_manager.append(data)
    }
}
