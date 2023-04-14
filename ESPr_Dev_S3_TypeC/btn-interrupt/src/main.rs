use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::Result;

use esp_idf_hal::{gpio::*, peripherals::Peripherals};

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio6)?;
    let mut button = PinDriver::input(peripherals.pins.gpio0)?;
    button.set_pull(Pull::Up)?;

    button.set_interrupt_type(InterruptType::NegEdge)?;

    unsafe {
        button.subscribe(move || {
            led.toggle().unwrap();
        })?;
    }

    println!("Hello, world!");

    loop {}
}
