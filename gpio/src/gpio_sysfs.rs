use std::fs;
use std::path::Path;

const GPIO_PATH: &str = "/sys/class/gpio";

pub struct GpioSysfs {
    gpio_num: u32,
}

impl GpioSysfs {
    pub fn new(gpio_num: u32) -> anyhow::Result<GpioSysfs> {
        GpioSysfs::export_gpio(gpio_num)?;

        Ok(GpioSysfs { gpio_num })
    }

    fn export_gpio(gpio_num: u32) -> anyhow::Result<()> {
        let path = format!("{GPIO_PATH}/export");

        Ok(fs::write(
            Path::new(&path),
            gpio_num.to_string().as_bytes(),
        )?)
    }

    pub fn set_gpio_direction(&self, direction: &str) -> anyhow::Result<()> {
        let path = format!("{GPIO_PATH}/gpio{}/direction", self.gpio_num);

        Ok(fs::write(Path::new(&path), direction.as_bytes())?)
    }

    pub fn write_gpio_value(&self, value: u8) -> anyhow::Result<()> {
        let path = format!("{GPIO_PATH}/gpio{}/value", self.gpio_num);

        Ok(fs::write(Path::new(&path), value.to_string().as_bytes())?)
    }

    pub fn read_gpio_value(&self) -> anyhow::Result<String> {
        let path = format!("{GPIO_PATH}/gpio{}/value", self.gpio_num);

        Ok(fs::read_to_string(Path::new(&path))?)
    }

    pub fn unexport_gpio(&self) -> anyhow::Result<()> {
        let path = format!("{GPIO_PATH}/unexport");

        Ok(fs::write(
            Path::new(&path),
            self.gpio_num.to_string().as_bytes(),
        )?)
    }
}

impl Drop for GpioSysfs {
    fn drop(&mut self) {
        let _ = self.set_gpio_direction("in");
        let _ = self.unexport_gpio();
    }
}
