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
use fugit::HertzU32;
pub use hal::{
    clock::Clocks,
    gpio::{InputPin, OutputPin, Pins, IO},
    i2c::{Instance as I2cInstance, I2C},
    peripheral::Peripheral,
    peripherals::Peripherals,
    prelude::*,
    spi::{
        master::{Instance as SpiInstance, Spi},
        FullDuplexMode,
        SpiMode,
    },
};

pub mod peripherals;

#[macro_export]
macro_rules! take_periph {
    () => {
        Peripherals::take()
    };
}

#[macro_export]
macro_rules! initialize_chip {
    ($peripherals:ident) => {{
        let system = $peripherals.SYSTEM.split();
        let clocks = ClockControl::max(system.clock_control).freeze();
        let io = IO::new($peripherals.GPIO, $peripherals.IO_MUX);

        // You can directly return the tuple from the macro
        (clocks, io.pins)
    }};
}

#[macro_export]
macro_rules! init_i2c_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        if cfg!(feature = "esp32") {
            I2C::new(
                $peripherals.I2C0,
                $pins.gpio32,
                $pins.gpio33,
                100u32.kHz(),
                &$clocks,
            )
        } else if cfg!(feature = "esp32s2") {
            I2C::new(
                $peripherals.I2C0,
                $pins.gpio7,
                $pins.gpio8,
                100u32.kHz(),
                &$clocks,
            )
        } else if cfg!(any(
            feature = "esp32s3",
            feature = "esp32c3",
            feature = "esp32c6",
            feature = "esp32h2"
        )) {
            {
                I2C::new(
                    $peripherals.I2C0,
                    $pins.gpio1,
                    $pins.gpio2,
                    100u32.kHz(),
                    &$clocks,
                )
            }
        } else {
            panic!("Unknown configuration");
        }
    };
}

#[macro_export]
macro_rules! init_i2c_custom {
    ($peripherals:ident, $clocks:ident, $sda_pin:expr, $scl_pin:expr, $freq:expr) => {
        I2C::new($peripherals.I2C0, $sda_pin, $scl_pin, $freq, &$clocks)
    };
}

#[macro_export]
macro_rules! init_spi_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        if cfg!(feature = "esp32") {
            Spi::new($peripherals.SPI2, 100u32.MHz(), SpiMode::Mode0, &$clocks).with_pins(
                Some($io.pins.gpio19),
                Some($io.pins.gpio23),
                Some($io.pins.gpio25),
                Some($io.pins.gpio22),
            )
        } else if cfg!(feature = "esp32s2") {
            Spi::new($peripherals.SPI2, 100u32.MHz(), SpiMode::Mode0, &$clocks).with_pins(
                Some($io.pins.gpio36),
                Some($io.pins.gpio35),
                Some($io.pins.gpio37),
                Some($io.pins.gpio34),
            )
        } else if cfg!(any(feature = "esp32c3", feature = "esp32c6")) {
            Spi::new($peripherals.SPI2, 100u32.MHz(), SpiMode::Mode0, &$clocks).with_pins(
                Some($io.pins.gpio6),
                Some($io.pins.gpio7),
                Some($io.pins.gpio5),
                Some($io.pins.gpio10),
            )
        } else if cfg!(feature = "esp32s3") {
            Spi::new($peripherals.SPI2, 100u32.MHz(), SpiMode::Mode0, &$clocks).with_pins(
                Some($io.pins.gpio12),
                Some($io.pins.gpio13),
                Some($io.pins.gpio11),
                Some($io.pins.gpio10),
            )
        } else if cfg!(feature = "esp32h2") {
            Spi::new($peripherals.SPI2, 100u32.MHz(), SpiMode::Mode0, &$clocks).with_pins(
                Some($io.pins.gpio1),
                Some($io.pins.gpio3),
                Some($io.pins.gpio2),
                Some($io.pins.gpio11),
            )
        } else {
            panic!("Unknown configuration")
        }
    };
}

#[macro_export]
macro_rules! init_spi_custom {
    ($peripherals:ident, $clocks:ident, $clk:expr, $mosi:expr, $miso:expr, $cs:expr, $freq:expr) => {
        Spi::new($peripherals.SPI2, $freq, SpiMode::Mode0, &$clocks)
            .with_pins($clk, $mosi, $miso, $cs)
    };
}

#[macro_export]
macro_rules! wait {
    ($delay:ident, $time:expr) => {
        $delay.delay_ms($time as u32);
    };
}

// Define an error type for initialization errors
pub enum InitError {
    UnsupportedChip,
    PeripheralsError,
    ClockError,
    GpioError,
    // ... other potential error types
}
