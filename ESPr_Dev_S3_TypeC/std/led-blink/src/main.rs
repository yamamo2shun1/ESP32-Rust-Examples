use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::Result;

use esp_idf_hal::{delay::Ets, gpio::*, peripherals::Peripherals};

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio6)?;

    println!("Hello, world!");

    loop {
        led.set_high()?;
        Ets::delay_ms(500);

        led.set_low()?;
        Ets::delay_ms(500);
    }
}
