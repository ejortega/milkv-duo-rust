use std::path::Path;

use anyhow::{anyhow, Result};

pub mod gpio_mmap;
pub mod gpio_sysfs;
pub mod milkv_duo;

pub trait FileSystemOps {
    fn write(&self, path: &Path, content: &[u8]) -> Result<()>;
    fn read_to_string(&self, path: &Path) -> Result<String>;
}

const GPIO_BASE_ADDRESS: usize = 0x03020000;
pub const GPIO0_BASE: usize = GPIO_BASE_ADDRESS;
pub const GPIO1_BASE: usize = GPIO_BASE_ADDRESS + 0x1000;
pub const GPIO2_BASE: usize = GPIO_BASE_ADDRESS + 0x2000;
pub const GPIO3_BASE: usize = GPIO_BASE_ADDRESS + 0x3000;

#[derive(Debug)]
#[repr(C)]
pub struct MilkVDuoGpioRegisters {
    swporta_dr: u32,
    swporta_ddr: u32,
    __reserved1: [u32; 10],
    inten: u32,
    intmask: u32,
    inttype_level: u32,
    int_polarity: u32,
    intstatus: u32,
    raw_intstatus: u32,
    debounce: u32,
    porta_eoi: u32,
    ext_porta: u32,
    __reserved2: [u32; 3],
    ls_sync: u32,
}

impl MilkVDuoGpioRegisters {
    pub fn new(base: usize) -> Result<&'static Self> {
        if base == 0 {
            return Err(anyhow!("Base cannot be NULL"));
        }

        let gpio_base = base as *const MilkVDuoGpioRegisters;

        unsafe {
            if gpio_base.is_null() {
                Err(anyhow!("Base is NULL"))
            } else {
                Ok(&*gpio_base)
            }
        }
    }

    pub fn swporta_dr(&self) -> usize {
        &self.swporta_dr as *const _ as _
    }

    pub fn swporta_ddr(&self) -> usize {
        &self.swporta_ddr as *const _ as _
    }

    pub fn inten(&self) -> usize {
        &self.inten as *const _ as _
    }

    pub fn intmask(&self) -> usize {
        &self.intmask as *const _ as _
    }

    pub fn inttype_level(&self) -> usize {
        &self.inttype_level as *const _ as _
    }

    pub fn int_polarity(&self) -> usize {
        &self.int_polarity as *const _ as _
    }

    pub fn intstatus(&self) -> usize {
        &self.intstatus as *const _ as _
    }

    pub fn raw_intstatus(&self) -> usize {
        &self.raw_intstatus as *const _ as _
    }

    pub fn debounce(&self) -> usize {
        &self.debounce as *const _ as _
    }

    pub fn porta_eoi(&self) -> usize {
        &self.porta_eoi as *const _ as _
    }

    pub fn ext_porta(&self) -> usize {
        &self.ext_porta as *const _ as _
    }

    pub fn ls_sync(&self) -> usize {
        &self.ls_sync as *const _ as _
    }
}
