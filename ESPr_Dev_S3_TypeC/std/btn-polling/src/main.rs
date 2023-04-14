use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::Result;

use esp_idf_hal::{delay::Ets, gpio::*, peripherals::Peripherals};

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut btn = PinDriver::input(peripherals.pins.gpio0)?;
    btn.set_pull(Pull::Up)?;

    let mut is_btn_low = false;

    println!("Hello, world!");

    loop {
        if btn.is_low() && !is_btn_low {
            println!("Btn: Pushed!");

            is_btn_low = true;
        } else if btn.is_high() && is_btn_low {
            println!("Btn: Released.");

            is_btn_low = false;
        }

        Ets::delay_ms(100);
    }
}
