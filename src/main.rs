#![no_main]
#![no_std]

// Build/flash instructions:
// objcopy --release -- -O ihex build.hex
// nrfutil pkg generate --hw-version 52 --sd-req=0x00 --application ./build.hex --application-version 1 prog.zip
// nrfutil dfu usb-serial -pkg prog.zip -p COM5

use embedded_hal::digital::v2::{InputPin, OutputPin};
use hal::{gpio::Level, pac::Peripherals, pac::RADIO};
use nrf52840_hal as hal;
use nrf52840_hal::Delay;
use rust_nrf52_bluetooth as _; // global logger + panicking-behavior + memory layout
use cortex_m::interrupt::Mutex;

use rubble::beacon::Beacon;
use rubble::link::{ad_structure::AdStructure, MIN_PDU_BUF};
use rubble_nrf5x::radio::{BleRadio, PacketBuffer};
use rubble_nrf5x::utils::get_device_address;
use core::borrow::BorrowMut;

// static BLE_TX_BUF: Mutex<[u8; 39]> = Mutex::new([0; 39]);
// static BLE_RX_BUF: Mutex<[u8; 39]> = Mutex::new([0; 39]);

static mut BLE_TX_BUF: PacketBuffer = [0; 39];
static mut BLE_RX_BUF: PacketBuffer = [0; 39];


#[cortex_m_rt::entry]
unsafe fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let port1 = hal::gpio::p1::Parts::new(peripherals.P1);

    let mut _delay = hal::timer::Timer::new(peripherals.TIMER0);

    let button = port1.p1_06.into_pullup_input();
    let mut led = port0.p0_06.into_push_pull_output(Level::High);

    // TODO: setup UART connection for serial printf debugging

    let _clocks = hal::clocks::Clocks::new(peripherals.CLOCK).enable_ext_hfosc();

    let device_address = get_device_address();

    // let mut ble_tx_buf: [u8; 39] = [0; 39];
    // let mut ble_tx_buf: PacketBuffer = [0; 39];
    // let mut ble_rx_buf: PacketBuffer = [0; 39];

    // Rubble currently requires an RX buffer even though the radio is only used as a TX-only beacon.
    let mut radio = BleRadio::new(
        peripherals.RADIO,
        &peripherals.FICR,
        &mut BLE_TX_BUF,
        &mut BLE_RX_BUF,
    );

    let beacon = Beacon::new(
        device_address,
        &[AdStructure::CompleteLocalName("Jarrod rust beacon")],
    )
    .unwrap();

    loop {
        _delay.delay(1_000_000);
        led.set_high().unwrap();

        beacon.broadcast(&mut radio);

        _delay.delay(1_000_000);
        led.set_low().unwrap();
        // if button.is_high().unwrap() {
        //     led.set_high().unwrap();
        // } else {
        //     led.set_low().unwrap();
        // }
    }
}
