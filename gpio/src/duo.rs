use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};

use crate::gpio_mmap::DevMem;
use crate::GpioDirection::GpioInput;
use crate::{FileSystemOps, Gpio, GpioDirection, GpioPort, IntLevelType, IntPolarity};

pub struct DuoFileSystem;

impl FileSystemOps for DuoFileSystem {
    fn write(&self, path: &Path, content: &[u8]) -> Result<()> {
        Ok(fs::write(path, content)?)
    }

    fn read_to_string(&self, path: &Path) -> Result<String> {
        Ok(fs::read_to_string(path)?)
    }
}

const GPIO_BASE_ADDRESS: usize = 0x03020000;
pub const GPIO0_BASE: usize = GPIO_BASE_ADDRESS;
pub const GPIO1_BASE: usize = GPIO_BASE_ADDRESS + 0x1000;
pub const GPIO2_BASE: usize = GPIO_BASE_ADDRESS + 0x2000;
pub const GPIO3_BASE: usize = GPIO_BASE_ADDRESS + 0x3000;
pub const PWR_GPIO_BASE: usize = 0x05021000;

impl GpioPort {
    pub fn base_address(&self) -> usize {
        match self {
            GpioPort::Port0 => GPIO0_BASE,
            GpioPort::Port1 => GPIO1_BASE,
            GpioPort::Port2 => GPIO2_BASE,
            GpioPort::Port3 => GPIO3_BASE,
            GpioPort::Pwr => PWR_GPIO_BASE,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct DuoGpio {
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

impl DuoGpio {
    pub fn new(base: usize) -> Result<&'static Self> {
        if base == 0 {
            return Err(anyhow!("Base cannot be NULL"));
        }

        let gpio_base = base as *const DuoGpio;

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

pub struct MilkVDuoGpio<'a> {
    pin: u32,
    bitmask: u32,
    duo: &'a DuoGpio,
    dev: DevMem,
}

impl<'a> Gpio for MilkVDuoGpio<'a> {
    fn new(port: GpioPort, pin: u32) -> Result<Self> {
        let bitmask = 1 << pin;
        let duo = DuoGpio::new(port.base_address())?;
        let dev = DevMem::new()?;
        Ok(Self { pin, bitmask, duo, dev })
    }

    fn init(&self, pin_direction: GpioDirection) -> Result<()> {
        let swporta_ddr = self.duo.swporta_ddr();
        let mut swporta_ddr_val = self.dev.mem_read(swporta_ddr)?;

        match pin_direction {
            GpioDirection::GpioInput => swporta_ddr_val &= !self.bitmask,
            GpioDirection::GpioOutput => swporta_ddr_val |= self.bitmask,
        };

        self.dev.mem_write(swporta_ddr, swporta_ddr_val)
    }

    fn write_pin(&self, pin_state: bool) -> Result<()> {
        let swporta_dr = self.duo.swporta_dr();
        let mut swporta_dr_val = self.dev.mem_read(swporta_dr)?;

        match pin_state {
            false => swporta_dr_val &= !self.bitmask,
            true => swporta_dr_val |= self.bitmask,
        };

        self.dev.mem_write(swporta_dr, swporta_dr_val)
    }

    fn read_pin(&self) -> Result<u32> {
        let swporta_dr = self.duo.swporta_dr();
        let swporta_dr_val = self.dev.mem_read(swporta_dr)? & self.bitmask;

        if swporta_dr_val > 0 {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn enable_interrupt(&self) -> Result<()> {
        let inten = self.duo.inten();
        self.enable(inten)
    }

    fn disable_interrupt(&self) -> Result<()> {
        let inten = self.duo.inten();
        self.disable(inten)
    }

    fn enable_interrupt_mask(&self) -> Result<()> {
        let intmask = self.duo.intmask();
        self.enable(intmask)
    }

    fn disable_interrupt_mask(&self) -> Result<()> {
        let intmask = self.duo.intmask();
        self.disable(intmask)
    }

    fn enable_debounce(&self) -> Result<()> {
        let debounce = self.duo.debounce();
        self.enable(debounce)
    }

    fn disable_debounce(&self) -> Result<()> {
        let debounce = self.duo.debounce();
        self.disable(debounce)
    }

    fn set_interrupt_level_type(&self, level_type: IntLevelType) -> Result<()> {
        let inttype_level = self.duo.inttype_level();
        let mut inttype_level_val = self.dev.mem_read(inttype_level)?;

        match level_type {
            IntLevelType::LevelSensitive => inttype_level_val &= !self.bitmask,
            IntLevelType::EdgeSensitive => inttype_level_val |= self.bitmask,
        }

        self.dev.mem_write(inttype_level, inttype_level_val)
    }

    fn set_interrupt_polarity(&self, polarity: IntPolarity) -> Result<()> {
        let int_polarity = self.duo.int_polarity();
        let mut int_polarity_val = self.dev.mem_read(int_polarity)?;

        match polarity {
            IntPolarity::ActiveLow => int_polarity_val &= !self.bitmask,
            IntPolarity::ActiveHigh => int_polarity_val |= self.bitmask,
        };

        self.dev.mem_write(int_polarity, int_polarity_val)
    }

    fn enable(&self, addr: usize) -> Result<()> {
        let mut val = self.dev.mem_read(addr)?;
        val |= self.bitmask;

        self.dev.mem_write(addr, val)
    }

    fn disable(&self, addr: usize) -> Result<()> {
        let mut val = self.dev.mem_read(addr)?;
        val &= !self.bitmask;

        self.dev.mem_write(addr, val)
    }
}

impl<'a> Drop for MilkVDuoGpio<'a> {
    fn drop(&mut self) {
        if let Err(e) = self.init(GpioInput) {
            log::error!("Error: {e}, unable to reset pin: {}", self.pin)
        }
    }
}
