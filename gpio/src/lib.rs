use std::path::Path;

use anyhow::Result;

pub mod duo;
pub mod gpio;
pub mod gpio_sysfs;

pub trait FileSystemOps {
    fn write(&self, path: &Path, content: &[u8]) -> Result<()>;
    fn read_to_string(&self, path: &Path) -> Result<String>;
}

pub enum Device {
    Duo,
}

pub enum GpioPort {
    Port0,
    Port1,
    Port2,
    Port3,
    Pwr,
}

pub enum GpioDirection {
    GpioInput,
    GpioOutput,
}

#[derive(Default)]
pub enum IntLevelType {
    LevelSensitive,
    #[default]
    EdgeSensitive,
}

#[derive(Default)]
pub enum IntPolarity {
    ActiveLow,
    #[default]
    ActiveHigh,
}

pub trait Gpio {
    fn new(port: GpioPort, pin: u32) -> Result<Self>
    where
        Self: Sized;
    fn init(&self, pin_direction: GpioDirection) -> Result<()>;
    fn write_pin(&self, pin_state: bool) -> Result<()>;
    fn read_pin(&self) -> Result<u32>;
    fn enable_interrupt(&self) -> Result<()>;
    fn disable_interrupt(&self) -> Result<()>;
    fn enable_interrupt_mask(&self) -> Result<()>;
    fn disable_interrupt_mask(&self) -> Result<()>;
    fn enable_debounce(&self) -> Result<()>;
    fn disable_debounce(&self) -> Result<()>;
    fn set_interrupt_level_type(&self, level_type: IntLevelType) -> Result<()>;
    fn set_interrupt_polarity(&self, polarity: IntPolarity) -> Result<()>;
    fn enable(&self, addr: usize) -> Result<()>;
    fn disable(&self, addr: usize) -> Result<()>;
}
