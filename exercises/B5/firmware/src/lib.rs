#![no_std]

use panic_probe as _;

pub mod uarte;
pub use nrf52840_hal as hal;
