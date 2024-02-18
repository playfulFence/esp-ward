#![no_std]

// Import the necessary modules from `esp-hal`
#[cfg(feature = "esp32")]
use esp32_hal as hal;
#[cfg(feature = "esp32c3")]
use esp32c3_hal as hal;
#[cfg(feature = "esp32c6")]
use esp32c6_hal as hal;
#[cfg(feature = "esp32h2")]
use esp32h2_hal as hal;
#[cfg(feature = "esp32s2")]
use esp32s2_hal as hal;
#[cfg(feature = "esp32s3")]
use esp32s3_hal as hal;

use hal::{clock::*, gpio::*, prelude::*};

// Define a generic initialization function that is dependent on a feature flag for the chip
pub fn initialize_chip() -> Result<ChipConfig, InitError> {
    let peripherals = hal::peripherals::Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = hal::clock::ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    Ok(ChipConfig {
        clocks,
        gpio: io.pins,
        peripheral_access: peripherals,
    })
}

// Define what the `ChipConfig` might look like
pub struct ChipConfig {
    // The fields here represent the peripherals that have been initialized
    pub clocks: hal::clock::Clocks<'static>,
    pub gpio: hal::gpio::Pins,
    peripheral_access: hal::peripherals::Peripherals,
}

// impl ChipConfig {
//     pub fn get_peripheral(&self) -> hal::peripherals::Peripherals {
//         self.peripheral_access;
//     }
// }

// Define an error type for initialization errors
pub enum InitError {
    UnsupportedChip,
    PeripheralsError,
    ClockError,
    GpioError,
    // ... other potential error types
}
