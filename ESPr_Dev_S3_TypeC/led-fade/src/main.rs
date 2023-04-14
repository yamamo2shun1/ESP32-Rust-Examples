use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::sync::Arc;

use anyhow::Result;

use esp_idf_hal::{
    delay::Ets,
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver},
    peripherals::Peripherals,
    prelude::*,
};

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let config = TimerConfig::default().frequency(25.kHz().into());
    let timer_driver = Arc::new(LedcTimerDriver::new(peripherals.ledc.timer0, &config)?);
    let mut driver = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver.clone(),
        peripherals.pins.gpio6,
    )?;
    //let mut led = PinDriver::output(peripherals.pins.gpio6)?;

    println!("Hello, world!");

    let max_duty = driver.get_max_duty();
    println!("max_duty = {}", max_duty);

    let max_count = 100;
    let mut count = 0;
    let mut is_dec = false;
    loop {
        driver.set_duty(max_duty * count / max_count)?;

        if is_dec {
            count -= 1;
        } else {
            count += 1;
        }

        if count > max_count {
            is_dec = true;
        } else if count <= 0 {
            is_dec = false;
        }

        Ets::delay_ms(10);
    }
}
