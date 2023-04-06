use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::Result;

// for use delay_ms
use esp_idf_hal::delay::Ets;

// for control onboald NeoPixel LED(SK6812)
use smart_leds_trait::{SmartLedsWrite, White};
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrbw32;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGBW8};

const LED_PIN: u32 = 2;
const NUM_PIXELS: usize = 1;

struct Status {
    sk6812: LedPixelEsp32Rmt<RGBW8, LedPixelColorGrbw32>,
}

impl Status {
    fn write(&mut self, r: u8, g: u8, b: u8) -> Result<()> {
        let pixels = std::iter::repeat(RGBW8::from((r, g, b, White(0)))).take(NUM_PIXELS);
        self.sk6812.write(pixels)?;

        Ok(());
    }

    fn clear(&mut self) -> Result<()> {
        let pixels = std::iter::repeat(RGBW8::from((0, 0, 0, White(0)))).take(NUM_PIXELS);
        self.sk6812.write(pixels)?;

        Ok(());
    }
}

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let mut status = Status {
        sk6812: LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(0, LED_PIN)?,
    };

    loop {
        status.write(255, 0, 0)?;
        Ets::delay_ms(500);

        status.clear()?;
        Ets::delay_ms(500);
    }
}
