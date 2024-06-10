use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use gpio::duo::{DuoGpio, GPIO2_BASE};
use gpio::gpio::DevMem;
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let should_terminate = Arc::new(AtomicBool::new(false));
    setup_signal_handler(should_terminate.clone())?;

    let dev = DevMem::new()?;
    let gpio = DuoGpio::new(GPIO2_BASE)?;
    let swporta_ddr = gpio.swporta_ddr();
    tracing::info!("SWPORTA_DDR Address: {:#010x}", swporta_ddr);
    let swporta_dr = gpio.swporta_dr();
    tracing::info!("SWPORTA_DR Address: {:#010x}", swporta_dr);

    // Enable LED (set GPIO to output mode)
    let mut val = dev.mem_read(swporta_ddr)?;
    tracing::info!("Initial SWPORTA_DDR Value: {:#010x}", val);
    val |= 1 << 24;
    dev.mem_write(swporta_ddr, val)?;

    let mut read_val;

    while !should_terminate.load(Ordering::SeqCst) {
        // Turn ON LED
        let mut status = dev.mem_read(swporta_dr)?;
        tracing::info!("SWPORTA_DR: Before Turning ON: {:#010x}", status);
        status |= 1 << 24;
        dev.mem_write(swporta_dr, status)?;
        read_val = dev.mem_read(swporta_dr)?;
        tracing::info!("SWPORTA_DR: After Turning ON: {:#010x}", read_val);
        sleep(Duration::from_secs(1));

        // Turn OFF LED
        status &= !(1 << 24);
        dev.mem_write(swporta_dr, status)?;
        read_val = dev.mem_read(swporta_dr)?;
        tracing::info!("SWPORTA_DR: After Turning OFF: {:#010x}", read_val);
        sleep(Duration::from_secs(1));
    }

    // Reset GPIO to input mode before exiting
    let mut val = dev.mem_read(swporta_ddr)?;
    val &= !(1 << 24);
    dev.mem_write(swporta_ddr, val)?;
    tracing::info!("Final SWPORTA_DDR Value: {:#010x}", val);

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
