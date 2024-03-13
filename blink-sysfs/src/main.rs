use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use gpio::gpio_sysfs::GpioSysfs;
use gpio::milkv_duo::DuoFileSystem;
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let should_terminate = Arc::new(AtomicBool::new(false));
    setup_signal_handler(should_terminate.clone())?;

    // milk-v duo 64m.
    let gpio_pin = 440;

    // milk-v duo 256m.
    // let gpio_pin = 354;

    let gpio = GpioSysfs::new(gpio_pin, DuoFileSystem)?;
    gpio.set_pin_mode_output()?;

    while !should_terminate.load(Ordering::SeqCst) {
        gpio.write_gpio_value(1)?;
        log::info!("LED ON");
        std::thread::sleep(std::time::Duration::from_secs(1));

        gpio.write_gpio_value(0)?;
        log::info!("LED OFF");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}

fn setup_signal_handler(flag: Arc<AtomicBool>) -> anyhow::Result<()> {
    let mut signals = Signals::new([SIGINT])?;
    std::thread::spawn(move || {
        if signals.into_iter().next().is_some() {
            flag.store(true, Ordering::SeqCst);
        }
    });
    Ok(())
}
