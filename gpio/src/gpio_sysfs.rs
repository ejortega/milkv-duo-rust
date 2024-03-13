use std::path::Path;

use anyhow::Result;

use crate::FileSystemOps;

const GPIO_PATH: &str = "/sys/class/gpio";
const DIRECTION_IN: &str = "in";
const DIRECTION_OUT: &str = "out";
const EXPORT: &str = "export";
const UNEXPORT: &str = "unexport";
const VALUE: &str = "value";
const DIRECTION: &str = "direction";

pub struct GpioSysfs<F: FileSystemOps> {
    gpio_pin: u32,
    gpio_label: String,
    fs_ops: F,
}

impl<F: FileSystemOps> GpioSysfs<F> {
    pub fn new(gpio_pin: u32, fs_ops: F) -> Result<Self> {
        let gpio_label = format!("gpio{gpio_pin}");
        let gpio = GpioSysfs { gpio_pin, gpio_label, fs_ops };
        gpio.export_gpio()?;

        Ok(gpio)
    }

    fn export_gpio(&self) -> Result<()> {
        let path = Path::new(GPIO_PATH).join(EXPORT);
        self.fs_ops.write(&path, self.gpio_pin.to_string().as_bytes())
    }

    fn set_gpio_direction(&self, direction: &str) -> Result<()> {
        let path = Path::new(GPIO_PATH).join(&self.gpio_label).join(DIRECTION);
        self.fs_ops.write(&path, direction.as_bytes())
    }

    pub fn set_pin_mode_input(&self) -> Result<()> {
        self.set_gpio_direction(DIRECTION_IN)
    }

    pub fn set_pin_mode_output(&self) -> Result<()> {
        self.set_gpio_direction(DIRECTION_OUT)
    }

    pub fn write_gpio_value(&self, value: u8) -> Result<()> {
        let path = Path::new(GPIO_PATH).join(&self.gpio_label).join(VALUE);
        self.fs_ops.write(&path, value.to_string().as_bytes())
    }

    pub fn read_gpio_value(&self) -> Result<String> {
        let path = Path::new(GPIO_PATH).join(&self.gpio_label).join(VALUE);
        self.fs_ops.read_to_string(&path)
    }

    pub fn unexport_gpio(&self) -> Result<()> {
        let path = Path::new(GPIO_PATH).join(UNEXPORT);
        self.fs_ops.write(&path, self.gpio_pin.to_string().as_bytes())
    }
}

impl<F: FileSystemOps> Drop for GpioSysfs<F> {
    fn drop(&mut self) {
        if let Err(e) = self.set_pin_mode_input() {
            log::error!("Error trying to reset direction: {e}");
        };
        if let Err(e) = self.unexport_gpio() {
            log::error!("Error trying to unexport pin {}: {e}", self.gpio_pin);
        };
    }
}
