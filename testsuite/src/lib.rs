#![no_std]
#![cfg_attr(test, no_main)]

use rust_nrf52_bluetooth as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
