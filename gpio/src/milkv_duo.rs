use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::FileSystemOps;

pub struct DuoFileSystem;

impl FileSystemOps for DuoFileSystem {
    fn write(&self, path: &Path, content: &[u8]) -> Result<()> {
        Ok(fs::write(path, content)?)
    }

    fn read_to_string(&self, path: &Path) -> Result<String> {
        Ok(fs::read_to_string(path)?)
    }
}
