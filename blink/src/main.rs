use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use gpio::duo::MilkVDuoGpio;
use gpio::Gpio;
use gpio::GpioDirection::GpioOutput;
use gpio::GpioPort::Port2;
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let should_terminate = Arc::new(AtomicBool::new(false));
    setup_signal_handler(should_terminate.clone())?;

    // https://milkv.io/docs/duo/getting-started/duo#duo-gpio-pinout
    // XGPIOC[24]
    let pin = 24;
    let duo_gpio: MilkVDuoGpio = Gpio::new(Port2, pin)?;

    // Enable LED (set GPIO to output mode)
    duo_gpio.init(GpioOutput)?;

    while !should_terminate.load(Ordering::SeqCst) {
        // Turn ON LED
        duo_gpio.write_pin(true)?;
        sleep(Duration::from_secs(1));

        // Turn OFF LED
        duo_gpio.write_pin(false)?;
        sleep(Duration::from_secs(1));
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
