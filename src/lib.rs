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

macro_rules! initialize_peripherals {
    ($peripherals:expr) => {{
        // Use the peripherals to set up the system, clocks, GPIO, etc.
        // This assumes that the `split` and initialization methods only borrow from peripherals
        let system_parts = $peripherals.SYSTEM.split();
        let clocks = ClockControl::boot_defaults(&system_parts.clock_control).freeze();

        let io = IO::new($peripherals.GPIO, $peripherals.IO_MUX);

        // Return a tuple or a struct that contains the initialized components
        // and a reference to the peripherals for further use
        Ok(ChipConfig {
            clocks,
            gpio: io.pins,
            $peripherals,
        })
    }};
}

// Define the `ChipConfig`
pub struct ChipConfig<'a> {
    // The fields here represent the peripherals that have been initialized
    pub clocks: hal::clock::Clocks<'static>,
    pub gpio: hal::gpio::Pins,
    pub periph: &'a hal::peripherals::Peripherals,
}

impl ChipConfig<'_> {
    pub fn get_peripheral(&self) -> &hal::peripherals::Peripherals {
        self.periph
    }
}

// Define an error type for initialization errors
pub enum InitError {
    UnsupportedChip,
    PeripheralsError,
    ClockError,
    GpioError,
    // ... other potential error types
}
