#![no_std]

use nrf52840_hal as _; // memory layout

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
// #[defmt::panic_handler]
// fn panic() -> ! {
//     cortex_m::asm::udf()
// }
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

// static COUNT: AtomicUsize = AtomicUsize::new(0);
// defmt::timestamp!("{=usize}", {
//     // NOTE(no-CAS) `timestamps` runs with interrupts disabled
//     let n = COUNT.load(Ordering::Relaxed);
//     COUNT.store(n + 1, Ordering::Relaxed);
//     n
// });
//
// /// Terminates the application and makes `probe-run` exit with exit-code = 0
// pub fn exit() -> ! {
//     loop {
//         cortex_m::asm::bkpt();
//     }
// }
