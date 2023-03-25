// src/storage/file_manager.rs

use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;

pub struct FileManager {
    file: File,
}

impl FileManager {
    pub fn new(file_path: &Path) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        Ok(Self { file })
    }

    pub fn read(&mut self, offset: u64, size: usize) -> std::io::Result<Vec<u8>> {
        let mut buffer = vec![0; size];
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    pub fn write(&mut self, offset: u64, data: &[u8]) -> std::io::Result<()> {
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(data)?;

        Ok(())
    }

    pub fn append(&mut self, data: &[u8]) -> std::io::Result<u64> {
        let offset = self.file.seek(SeekFrom::End(0))?;
        self.file.write_all(data)?;

        Ok(offset)
    }
}

