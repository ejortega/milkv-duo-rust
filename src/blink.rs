use std::fs;
use std::path::Path;

pub struct Gpio {
    gpio_num: u32,
}

impl Gpio {
    pub fn new(gpio_num: u32) -> anyhow::Result<Gpio> {
        Gpio::export_gpio(gpio_num)?;

        Ok(Gpio { gpio_num })
    }

    fn export_gpio(gpio_num: u32) -> anyhow::Result<()> {
        Ok(fs::write(
            "/sys/class/gpio/export",
            gpio_num.to_string().as_bytes(),
        )?)
    }

    pub fn set_gpio_direction(&self, direction: &str) -> anyhow::Result<()> {
        let path = format!("/sys/class/gpio/gpio{}/direction", self.gpio_num);
        Ok(fs::write(Path::new(&path), direction.as_bytes())?)
    }

    pub fn write_gpio_value(&self, value: u8) -> anyhow::Result<()> {
        let path = format!("/sys/class/gpio/gpio{}/value", self.gpio_num);
        Ok(fs::write(Path::new(&path), value.to_string().as_bytes())?)
    }

    #[allow(dead_code)]
    pub fn read_gpio_value(&self) -> anyhow::Result<String> {
        let path = format!("/sys/class/gpio/gpio{}/value", self.gpio_num);
        Ok(fs::read_to_string(Path::new(&path))?)
    }

    fn unexport_gpio(&self) -> anyhow::Result<()> {
        Ok(fs::write(
            "/sys/class/gpio/unexport",
            self.gpio_num.to_string().as_bytes(),
        )?)
    }
}

impl Drop for Gpio {
    fn drop(&mut self) {
        let _ = self.set_gpio_direction("in");
        let _ = self.unexport_gpio();
    }
}
