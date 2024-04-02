#![no_std]
#![feature(type_alias_impl_trait)]

//! # esp-ward
//!
//! `esp-ward` is a Rust crate designed as a higher-level abstraction over
//! `esp-hal` to simplify the usage of ESP32, ESP32S2, ESP32C3, ESP32C6,
//! ESP32S3, and ESP32H2 chips with Rust. It provides common APIs, traits, and
//! structs to interact with various peripherals such as GPIOs, I2C, and SPI
//! devices.
//!
//! This crate is targeted at developers new to the `esp-rs` ecosystem or those
//! who prefer a simplified interface for common operations.
//!
//! ## Features
//! - Traits and structs for common peripheral interactions.
//! - Easy configuration of SPI and I2C.
//! - Predefined macros for common operations and setup routines.
//! - Compatible with various ESP32 family chips.
//! - Simplified Wi-Fi and MQTT features
//!
//! ## Usage
//! To use `esp-ward`, include it as a dependency in your `Cargo.toml` and refer
//! to the following examples to start interacting with your ESP device's
//! hardware features.
//!
//! ### Quick Start
//! Here's how you might initialize the system peripherals and configure I2C and
//! SPI with default settings:
//! ```rust
//! use esp_ward::{initialize_chip, take_periph, take_system};
//!
//! let peripherals = take_periph!();
//! let system = take_system!(peripherals);
//! let (clocks, pins) = initialize_chip!(peripherals, system);
//! // Now you can use `clocks` and `pins` to interact with the peripherals
//! ```
//!
//! ### Example: Configuring I2C
//! ```rust
//! # #[cfg(feature = "esp32")]
//! use esp_ward::init_i2c_default;
//!
//! let i2c = init_i2c_default!(peripherals, pins, clocks);
//! // Now `i2c` is ready to communicate with I2C devices
//! ```
//!
//! ### Example: Configuring SPI
//! ```rust
//! # #[cfg(feature = "esp32")]
//! use esp_ward::init_spi_default;
//!
//! let spi = init_spi_default!(peripherals, pins, clocks);
//! // Now `spi` is ready to transfer data with SPI devices
//! ```
//!
//! ## Macros
//! This crate also provides several macros to ease the setup and usage of ESP
//! peripherals.
//!
//! ### `take_periph`
//! Takes the peripherals from the ESP board. This is typically one of the first
//! steps in a Rust-ESP application.
//!
//! ### `take_system`
//! Splits the `SYSTEM` peripheral into its constituent parts.
//!
//! ### `initialize_chip`
//! Initializes the system clocks and IO pins.
//!
//! ### `init_i2c_default` and `init_i2c_custom`
//! Initializes the I2C peripheral with either default or custom configurations.
//!
//! ### `init_spi_default` and `init_spi_custom`
//! Initializes the SPI peripheral with either default or custom configurations.
//!
//! ### `init_wifi`
//! Initializes Wi-Fi connection in async or non-async way - depending on your
//! project
//!
//! and more...
//!
//! ## Contributing
//! Contributions to `esp-ward` are welcome. Check out the repository on GitHub
//! to report issues or submit pull requests.
//!
//! ## License
//! `esp-ward` is distributed under the terms of both the MIT license and the
//! Apache License (Version 2.0).
//!
//! See LICENSE-APACHE and LICENSE-MIT for details.

// Import the necessary modules from `esp-hal`
pub use esp_hal::{
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

pub mod connectivity;
pub mod display;
pub mod peripherals;
// TO BE FIXED (Blocked by "esp-wifi")
// pub mod tiny_mqtt;

/// Takes the ESP peripherals. This should be one of the first steps in an ESP
/// application, ensuring that the peripherals are properly acquired before use.
///
/// # Examples
/// ```no_run
/// let peripherals = esp_ward::take_periph!();
/// ```
#[macro_export]
macro_rules! take_periph {
    () => {
        esp_hal::peripherals::Peripherals::take()
    };
}

/// Splits the `SYSTEM` peripheral into its constituent parts.
/// This macro is a convenience wrapper for quickly accessing system components.
///
/// # Examples
/// ```no_run
/// let peripherals = esp_ward::take_periph!();
/// let system_parts = esp_ward::take_system!(peripherals);
/// ```
#[macro_export]
macro_rules! take_system {
    ($peripherals:ident) => {
        $peripherals.SYSTEM.split()
    };
}

/// Initializes the system clocks and IO pins, providing the base setup required
/// for any operation with peripherals.
///
/// Pins are accessible like `pins.gpio2`.
/// # Examples
/// ```no_run
/// let peripherals = esp_ward::take_periph!();
/// let system = esp_ward::take_system!(peripherals);
/// let (clocks, pins) = esp_ward::initialize_chip!(peripherals, system);
/// ```
#[macro_export]
macro_rules! initialize_chip {
    ($peripherals:ident, $system:ident) => {{
        use embedded_hal::blocking::delay::{DelayMs, DelayUs};
        use esp_hal::delay::Delay;
        let clocks = esp_hal::clock::ClockControl::boot_defaults($system.clock_control).freeze();
        let io = esp_hal::gpio::IO::new($peripherals.GPIO, $peripherals.IO_MUX);
        let mut delay = esp_hal::delay::Delay::new(&clocks);

        // You can directly return the tuple from the macro
        (clocks, io.pins, delay)
    }};
}

// `init_i2c_default` is defined separately for each chip feature, since
// different chips have different pin layouts. Include
// one of these for each `#[cfg(feature = "...")]` version.
/// Initializes the default I2C configuration for the ESP board.
/// Assumes the use of the standard I2C0 peripheral and "default" pin
/// configuration. The rest of "default" functions
// and macros were desinged in a way to avoid collisions, so you're able
/// # Examples
/// ```no_run
/// let peripherals = esp_ward::take_periph!();
/// let (clocks, pins) = esp_ward::initialize_chip!(peripherals);
/// let mut i2c = esp_ward::init_i2c_default!(peripherals, pins, clocks);
/// ```
#[cfg(any(
    feature = "esp32c3",
    feature = "esp32c6",
    feature = "esp32s3",
    feature = "esp32h2"
))]
#[macro_export]
macro_rules! init_i2c_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        esp_hal::i2c::I2C::new(
            $peripherals.I2C0,
            $pins.gpio1,
            $pins.gpio2,
            100u32.kHz(),
            &$clocks,
        )
    };
}

#[cfg(feature = "esp32")]
#[macro_export]
macro_rules! init_i2c_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        esp_hal::i2c::I2C::new(
            $peripherals.I2C0,
            $pins.gpio32,
            $pins.gpio33,
            100u32.kHz(),
            &$clocks,
        )
    };
}

#[cfg(feature = "esp32s2")]
#[macro_export]
macro_rules! init_i2c_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        esp_hal::i2c::I2C::new(
            $peripherals.I2C0,
            $pins.gpio7,
            $pins.gpio8,
            100u32.kHz(),
            &$clocks,
        )
    };
}

/// Initializes a custom I2C configuration, allowing for arbitrary SDA and SCL
/// pins and frequency.
///
/// # Arguments
/// * `$peripherals`: The peripherals instance taken from the board.
/// * `$clocks`: The system clocks initialized beforehand.
/// * `$sda_pin`: The pin to use for SDA.
/// * `$scl_pin`: The pin to use for SCL.
/// * `$freq`: The frequency for I2C communication.
///
/// # Examples
/// ```no_run
/// let peripherals = esp_ward::take_periph!();
/// let system = esp_ward::take_system!(peripherals);
/// let (clocks, pins) = esp_ward::initialize_chip!(peripherals, system);
/// let mut i2c =
///     esp_ward::init_i2c_custom!(peripherals, &clocks, pins.gpio21, pins.gpio22, 100u32.kHz());
/// ```
#[macro_export]
macro_rules! init_i2c_custom {
    ($peripherals:ident, $clocks:ident, $sda_pin:expr, $scl_pin:expr, $freq:expr) => {
        I2C::new($peripherals.I2C0, $sda_pin, $scl_pin, $freq, &$clocks)
    };
}

/// Initializes the default SPI configuration for the ESP32.
/// Assumes the use of the standard SPI2 peripheral and default pin
/// configuration.
///
/// # Examples
/// ```no_run
/// let peripherals = esp_ward::take_periph!();
/// let (clocks, pins) = esp_ward::initialize_chip!(peripherals);
/// let spi = esp_ward::init_spi_default!(peripherals, pins, clocks);
/// ```
#[cfg(any(feature = "esp32c3", feature = "esp32c6"))]
#[macro_export]
macro_rules! init_spi_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        esp_hal::spi::master::Spi::new(
            $peripherals.SPI2,
            100u32.MHz(),
            esp_hal::spi::SpiMode::Mode0,
            &$clocks,
        )
        .with_pins(
            Some($pins.gpio6),
            Some($pins.gpio7),
            Some($pins.gpio5),
            Some($pins.gpio10),
        )
    };
}

#[cfg(feature = "esp32")]
#[macro_export]
macro_rules! init_spi_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        esp_hal::spi::master::Spi::new(
            $peripherals.SPI2,
            100u32.MHz(),
            esp_hal::spi::SpiMode::Mode0,
            &$clocks,
        )
        .with_pins(
            Some($pins.gpio19),
            Some($pins.gpio23),
            Some($pins.gpio25),
            Some($pins.gpio22),
        )
    };
}

#[cfg(feature = "esp32s2")]
#[macro_export]
macro_rules! init_spi_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        esp_hal::spi::master::Spi::new(
            $peripherals.SPI2,
            100u32.MHz(),
            esp_hal::spi::SpiMode::Mode0,
            &$clocks,
        )
        .with_pins(
            Some($pins.gpio36),
            Some($pins.gpio35),
            Some($pins.gpio37),
            Some($pins.gpio34),
        )
    };
}

#[cfg(feature = "esp32s3")]
#[macro_export]
macro_rules! init_spi_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        esp_hal::spi::master::Spi::new(
            $peripherals.SPI2,
            100u32.MHz(),
            esp_hal::spi::SpiMode::Mode0,
            &$clocks,
        )
        .with_pins(
            Some($pins.gpio12),
            Some($pins.gpio13),
            Some($pins.gpio11),
            Some($pins.gpio10),
        )
    };
}

#[cfg(feature = "esp32h2")]
#[macro_export]
macro_rules! init_spi_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        esp_hal::spi::master::Spi::new(
            $peripherals.SPI2,
            100u32.MHz(),
            esp_hal::spi::SpiMode::Mode0,
            &$clocks,
        )
        .with_pins(
            Some($pins.gpio1),
            Some($pins.gpio3),
            Some($pins.gpio2),
            Some($pins.gpio11),
        )
    };
}

/// Initializes a custom SPI configuration, allowing for arbitrary CLK, MOSI,
/// MISO, and CS pins and frequency.
///
/// # Arguments
/// * `$peripherals`: The peripherals instance taken from the board.
/// * `$clocks`: The system clocks initialized beforehand.
/// * `$clk`: The pin to use for CLK.
/// * `$mosi`: The pin to use for MOSI.
/// * `$miso`: The pin to use for MISO.
/// * `$cs`: The pin to use for CS.
/// * `$freq`: The frequency for SPI communication.
///
/// # Examples
/// ```no_run
/// let peripherals = esp_ward::take_periph!();
/// let (clocks, pins) = esp_ward::initialize_chip!(peripherals);
/// let spi = esp_ward::init_spi_custom!(
///     peripherals,
///     clocks,
///     pins.gpio18,
///     pins.gpio23,
///     pins.gpio19,
///     pins.gpio5,
///     100u32.MHz()
/// );
/// ```
#[macro_export]
macro_rules! init_spi_custom {
    ($peripherals:ident, $clocks:ident, $clk:expr, $mosi:expr, $miso:expr, $cs:expr, $freq:expr) => {
        esp_hal::spi::master::Spi::new(
            $peripherals.SPI2,
            $freq,
            esp_hal::spi::SpiMode::Mode0,
            &$clocks,
        )
        .with_pins($clk, $mosi, $miso, $cs)
    };
}

/// Pauses the execution for a specified number of milliseconds using a delay
/// provider.
///
/// # Arguments
/// * `$delay`: The delay provider, typically from a HAL implementation.
/// * `$time`: The number of milliseconds to pause.
///
/// # Examples
/// ```no_run
/// let mut delay = esp_hal::Delay::new();
/// esp_ward::wait!(delay, 1000); // pauses for 1 second
/// ```
#[macro_export]
macro_rules! wait {
    ($delay:ident, $time:expr) => {
        use embedded_hal::blocking::delay::DelayMs;
        $delay.delay_ms($time as u32);
    };
}

/// Sets up a global allocator for heap memory, required for the `alloc` crate
/// functionalities. This is essential for using heap-allocated data structures
/// which are used, for example, for `max7219` display.
/// ATTENTION: MAKE SURE to use this `prepare_alloc` as a first function in your
/// program if you're using module which utilizes `alloc`. Modules like this
/// will have a warning that you should use the `alloc` feature
///
/// # Safety
/// This macro should be called <b>ONCE AND ONLY ONCE</b> during initialization
/// before using any features that require dynamic memory allocation.
///
/// # Examples
/// ```no_run
/// esp_ward::prepare_alloc!();
/// let v: Vec<u8> = Vec::new(); // Now we can use structs from `alloc`, like `Vec``
/// ```
#[macro_export]
macro_rules! prepare_alloc {
    () => {
        extern crate alloc;
        #[global_allocator]
        static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

        const HEAP_SIZE: usize = 32 * 1024;
        static mut HEAP: core::mem::MaybeUninit<[u8; HEAP_SIZE]> = core::mem::MaybeUninit::uninit();

        unsafe {
            ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
        }
    };
}
