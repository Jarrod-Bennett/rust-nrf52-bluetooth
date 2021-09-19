#![no_main]
#![no_std]

// Build/flash instructions:
// objcopy --release -- -O ihex build.hex
// nrfutil pkg generate --hw-version 52 --sd-req=0x00 --application ./build.hex --application-version 1 prog.zip
// nrfutil dfu usb-serial -pkg prog.zip -p COM5

use rust_nrf52_bluetooth as _; // global logger + panicking-behavior + memory layout
use nrf52840_hal as hal;
use hal::{
    pac::Peripherals,
    gpio::Level,
};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use nrf52840_hal::Delay;
use embedded_hal::blocking::delay::DelayMs;
use core::borrow::Borrow;

#[cortex_m_rt::entry]
fn main() -> ! {

    let peripherals = Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let port1 = hal::gpio::p1::Parts::new(peripherals.P1);

    let mut _delay = hal::timer::Timer::new(peripherals.TIMER0);

    let button = port1.p1_06.into_pullup_input();
    let mut led = port0.p0_06.into_push_pull_output(Level::High);

    // TODO: setup UART connection for serial printf debugging

    loop {
        if button.is_high().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
    }
}
