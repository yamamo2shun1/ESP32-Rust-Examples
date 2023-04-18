#![no_std]
#![no_main]

use core::f32;

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl,
    ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        LSGlobalClkSource, LowSpeed, LEDC,
    },
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Delay, Rtc, IO,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    println!("Hello world!");

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let led = io.pins.gpio6.into_push_pull_output();

    let mut ledc = LEDC::new(
        peripherals.LEDC,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty8Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 24u32.kHz(),
        })
        .unwrap();

    let mut channel0 = ledc.get_channel(channel::Number::Channel0, led);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 0,
        })
        .unwrap();

    let mut delay = Delay::new(&clocks);

    let max_count = 128;
    let mut count = 0;
    let mut is_dec = false;

    loop {
        let duty = (100.0 * (count as f32 / max_count as f32)) as u8;
        channel0.set_duty(duty).unwrap();

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

        delay.delay_ms(10u32);
    }
}
