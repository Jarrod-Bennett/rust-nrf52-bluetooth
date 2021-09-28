#![no_main]
#![no_std]

// Build/flash instructions:
// objcopy --release -- -O ihex build.hex
// nrfutil pkg generate --hw-version 52 --sd-req=0x00 --application ./build.hex --application-version 1 prog.zip
// nrfutil dfu usb-serial -pkg prog.zip -p COM5

use rust_nrf52_bluetooth as _; // global logger + panicking behaviour + memory layout

#[rtic::app(device = nrf52840_hal::pac, dispatchers = [WDT, TIMER3])]
mod app {
    use dwt_systick_monotonic::DwtSystick;
    use rtic::time::duration::*;

    use nrf52840_hal as hal;
    use hal::{gpio::Level};
    use nrf52840_hal::gpio::{Output, PushPull};
    use nrf52840_hal::gpio::p0::P0_06;
    use nrf52840_hal::prelude::OutputPin;
    use nrf52840_hal::usbd::{Usbd, UsbPeripheral};

    use usb_device::{bus::UsbBusAllocator, prelude::*};
    use usbd_serial::{SerialPort};
    use nrf52840_hal::Clocks;
    use nrf52840_hal::clocks::{ExternalOscillator, Internal, LfOscStopped};

    #[monotonic(binds = SysTick, default = true)]
    type DwtMono = DwtSystick<64_000_000>;

    #[shared]
    struct Shared {
        led: P0_06<Output<PushPull>>,
    }

    #[local]
    struct Local {
        usb_dev: UsbDevice<'static, Usbd<UsbPeripheral<'static>>>,
        serial: SerialPort<'static, Usbd<UsbPeripheral<'static>>>,
    }

    #[init(local = [clocks: Option<Clocks<ExternalOscillator, Internal, LfOscStopped>> = None,
            usb_bus: Option<UsbBusAllocator<Usbd<UsbPeripheral<'static>>>> = None])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {

        let mut dcb = cx.core.DCB;
        let mono = DwtSystick::new(&mut dcb, cx.core.DWT, cx.core.SYST, hal::clocks::HFCLK_FREQ);

        let port0 = hal::gpio::p0::Parts::new(cx.device.P0);
        let led = port0.p0_06.into_push_pull_output(Level::High);

        let clocks = cx.local.clocks;
        clocks.replace(hal::clocks::Clocks::new(cx.device.CLOCK).enable_ext_hfosc());

        let usb_peripheral = UsbPeripheral::new(cx.device.USBD, clocks.as_ref().unwrap());
        let u = Usbd::new(usb_peripheral);

        let usb_bus = cx.local.usb_bus;
        usb_bus.replace(u);

        let serial = SerialPort::new(usb_bus.as_ref().unwrap());
        let usb_dev = UsbDeviceBuilder::new(usb_bus.as_ref().unwrap(),
                                            UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("Jarrod inc.")
            .product("Serial port")
            .serial_number("4")
            .device_class(usbd_serial::USB_CLASS_CDC)
            .max_packet_size_0(64) // (makes control transfers 8x faster)
            .build();

        // unsafe { hal::pac::NVIC::set_priority(cx.device.NVIC, hal::pac::Interrupt::USBD, 2) }
        // unsafe { hal::pac::NVIC::unmask(hal::pac::Interrupt::USBD) }

        task1::spawn().ok();

        ( Shared { led, }, Local { serial, usb_dev }, init::Monotonics(mono), )
    }

    // Optional idle, can be removed if not needed.
    // Note that removing this will put the MCU to sleep when no task is running, and this
    // generally breaks RTT based printing.
    #[idle(local = [serial, usb_dev])]
    fn idle(cx: idle::Context) -> ! {

        let serial = cx.local.serial;
        let usb_dev = cx.local.usb_dev;

        loop {
            if !usb_dev.poll(&mut [serial]) {
                continue;
            }

            let mut buf = [0u8; 64];

            match serial.read(&mut buf) {
                Ok(count) if count > 0 => {
                    // Echo back in upper case
                    for c in buf[0..count].iter_mut() {
                        if 0x61 <= *c && *c <= 0x7a {
                            *c &= !0x20;
                        }
                    }

                    let mut write_offset = 0;
                    while write_offset < count {
                        match serial.write(&buf[write_offset..count]) {
                            Ok(len) if len > 0 => {
                                write_offset += len;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    #[task(binds = USBD)]//, priority = 5, local = [serial, usb_dev])]
    fn usbd(_: usbd::Context) {

        // let serial = cx.local.serial;
        // let usb_dev = cx.local.usb_dev;
        //
        // // if !usb_dev.poll(&mut [serial]) {
        // //     return;
        // // }
        //
        // let mut buf = [0u8; 64];
        //
        // match serial.read(&mut buf) {
        //     Ok(count) if count > 0 => {
        //         // Echo back in upper case
        //         for c in buf[0..count].iter_mut() {
        //             if 0x61 <= *c && *c <= 0x7a {
        //                 *c &= !0x20;
        //             }
        //         }
        //
        //         let mut write_offset = 0;
        //         while write_offset < count {
        //             match serial.write(&buf[write_offset..count]) {
        //                 Ok(len) if len > 0 => {
        //                     write_offset += len;
        //                 }
        //                 _ => {}
        //             }
        //         }
        //     }
        //     _ => {}
        // }
    }

    #[task(shared = [led])]
    fn task1(mut cx: task1::Context) {
        cx.shared.led.lock(|led| {
            led.set_high().unwrap();
        });
        task2::spawn_after(1.seconds()).unwrap();
    }

    #[task(shared = [led])]
    fn task2(mut cx: task2::Context) {
        cx.shared.led.lock(|led| {
            led.set_low().unwrap();
        });
        task1::spawn_after(1.seconds()).unwrap();
    }
}
