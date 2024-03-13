use std::path::Path;

use anyhow::Result;

pub mod gpio_sysfs;
pub mod milkv_duo;

pub trait FileSystemOps {
    fn write(&self, path: &Path, content: &[u8]) -> Result<()>;
    fn read_to_string(&self, path: &Path) -> Result<String>;
}
