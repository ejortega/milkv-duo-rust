// tests/gpio_sysfs_tests.rs
use std::path::Path;

use anyhow::Result;
use gpio::FileSystemOps;
use mockall::mock;
use mockall::predicate::*;

// Define the mock for FileSystemOps using mockall
mock! {
    pub FileSystemOps {}
    impl FileSystemOps for FileSystemOps {
        fn write(&self, path: &Path, content: &[u8]) -> Result<()>;
        fn read_to_string(&self, path: &Path) -> Result<String>;
    }
}

#[cfg(test)]
mod tests {
    use gpio::gpio_sysfs::GpioSysfs;
    use mockall::predicate;

    use super::*;

    #[test]
    fn test_export_gpio() {
        let mut mock_fs = MockFileSystemOps::new();
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/export")),
                predicate::eq("17".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/gpio17/direction")),
                predicate::eq("in".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/unexport")),
                predicate::eq("17".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let _ = GpioSysfs::new(17, mock_fs).unwrap();
    }

    #[test]
    fn test_set_pin_mode_input() {
        let mut mock_fs = MockFileSystemOps::new();
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/export")),
                predicate::eq("17".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/gpio17/direction")),
                predicate::eq("in".as_bytes()),
            )
            .times(2)
            .returning(|_, _| Ok(()));
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/unexport")),
                predicate::eq("17".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let gpio = GpioSysfs::new(17, mock_fs).unwrap();
        gpio.set_pin_mode_input().unwrap();
    }

    #[test]
    fn test_write_gpio_value() {
        let mut mock_fs = MockFileSystemOps::new();
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/export")),
                predicate::eq("17".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/gpio17/value")),
                predicate::eq("1".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/gpio17/direction")),
                predicate::eq("in".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/unexport")),
                predicate::eq("17".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let gpio = GpioSysfs::new(17, mock_fs).unwrap();
        gpio.write_gpio_value(1).unwrap();
    }

    #[test]
    fn test_read_gpio_value() {
        let mut mock_fs = MockFileSystemOps::new();
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/export")),
                predicate::eq("17".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/gpio17/direction")),
                predicate::eq("in".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));
        mock_fs
            .expect_read_to_string()
            .with(predicate::eq(Path::new("/sys/class/gpio/gpio17/value")))
            .times(1)
            .returning(|_| Ok("1".to_string()));
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/unexport")),
                predicate::eq("17".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let gpio = GpioSysfs::new(17, mock_fs).unwrap();
        let value = gpio.read_gpio_value().unwrap();
        assert_eq!(value, "1");
    }

    #[test]
    fn test_unexport_gpio() {
        let mut mock_fs = MockFileSystemOps::new();
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/export")),
                predicate::eq("17".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/gpio17/direction")),
                predicate::eq("in".as_bytes()),
            )
            .times(1)
            .returning(|_, _| Ok(()));
        mock_fs
            .expect_write()
            .with(
                predicate::eq(Path::new("/sys/class/gpio/unexport")),
                predicate::eq("17".as_bytes()),
            )
            .times(2) // Expecting two calls to unexport, one explicitly and one from the drop
            .returning(|_, _| Ok(()));

        let gpio = GpioSysfs::new(17, mock_fs).unwrap();
        gpio.unexport_gpio().unwrap();
    }
}
