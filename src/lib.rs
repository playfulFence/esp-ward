#![no_std]

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

#[macro_export]
macro_rules! take_periph {
    () => {
        Peripherals::take()
    };
}

#[macro_export]
macro_rules! take_system {
    ($peripherals:ident) => {
        $peripherals.SYSTEM.split()
    };
}

#[macro_export]
macro_rules! initialize_chip {
    ($peripherals:ident, $system:ident) => {{
        let clocks = ClockControl::max($system.clock_control).freeze();
        let io = IO::new($peripherals.GPIO, $peripherals.IO_MUX);

        // You can directly return the tuple from the macro
        (clocks, io.pins)
    }};
}

#[cfg(feature = "esp32")]
#[macro_export]
macro_rules! init_i2c_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        I2C::new(
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
        I2C::new(
            $peripherals.I2C0,
            $pins.gpio7,
            $pins.gpio8,
            100u32.kHz(),
            &$clocks,
        )
    };
}

#[cfg(any(
    feature = "esp32c3",
    feature = "esp32c6",
    feature = "esp32s3",
    feature = "esp32h2"
))]
#[macro_export]
macro_rules! init_i2c_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        I2C::new(
            $peripherals.I2C0,
            $pins.gpio1,
            $pins.gpio2,
            100u32.kHz(),
            &$clocks,
        )
    };
}

#[macro_export]
macro_rules! init_i2c_custom {
    ($peripherals:ident, $clocks:ident, $sda_pin:expr, $scl_pin:expr, $freq:expr) => {
        I2C::new($peripherals.I2C0, $sda_pin, $scl_pin, $freq, &$clocks)
    };
}

#[cfg(feature = "esp32")]
#[macro_export]
macro_rules! init_spi_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        Spi::new($peripherals.SPI2, 100u32.MHz(), SpiMode::Mode0, &$clocks).with_pins(
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
        Spi::new($peripherals.SPI2, 100u32.MHz(), SpiMode::Mode0, &$clocks).with_pins(
            Some($pins.gpio36),
            Some($pins.gpio35),
            Some($pins.gpio37),
            Some($pins.gpio34),
        )
    };
}

#[cfg(any(feature = "esp32c3", feature = "esp32c6"))]
#[macro_export]
macro_rules! init_spi_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        Spi::new($peripherals.SPI2, 100u32.MHz(), SpiMode::Mode0, &$clocks).with_pins(
            Some($pins.gpio6),
            Some($pins.gpio7),
            Some($pins.gpio5),
            Some($pins.gpio10),
        )
    };
}

#[cfg(feature = "esp32s3")]
#[macro_export]
macro_rules! init_spi_default {
    ($peripherals:ident, $pins:ident, $clocks:ident) => {
        Spi::new($peripherals.SPI2, 100u32.MHz(), SpiMode::Mode0, &$clocks).with_pins(
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
        Spi::new($peripherals.SPI2, 100u32.MHz(), SpiMode::Mode0, &$clocks).with_pins(
            Some($pins.gpio1),
            Some($pins.gpio3),
            Some($pins.gpio2),
            Some($pins.gpio11),
        )
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

// TODO: revize how to make it smarter
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

// Define an error type for initialization errors
pub enum InitError {
    UnsupportedChip,
    PeripheralsError,
    ClockError,
    GpioError,
    // ... other potential error types
}
