use signal_hook::{consts::SIGINT, iterator::Signals};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod blink;
use crate::blink::Gpio;

#[tokio::main(flavor = "current_thread")]
#[tracing::instrument]
async fn main() -> anyhow::Result<()> {
    let should_terminate = Arc::new(AtomicBool::new(false));
    setup_signal_handler(should_terminate.clone())?;

    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    println!("Hello, world!");

    tracing::info!("Rust is the future");

    blink_led(should_terminate)?;

    Ok(())
}

fn blink_led(should_terminate: Arc<AtomicBool>) -> anyhow::Result<()> {
    // milk-v duo 64m.
    let gpio_num = 440;

    // milk-v duo 256m.
    // let gpio_num = 354;

    let gpio = Gpio::new(gpio_num)?;
    gpio.set_gpio_direction("out")?;

    while !should_terminate.load(Ordering::SeqCst) {
        gpio.write_gpio_value(1)?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        gpio.write_gpio_value(0)?;
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
