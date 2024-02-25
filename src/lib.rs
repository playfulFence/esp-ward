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
use hal::{
    clock::Clocks,
    gpio::{InputPin, OutputPin, Pins},
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

#[allow(unused_macros)]
macro_rules! initialize_peripherals {
    ($peripherals:expr) => {{
        // Use the peripherals to set up the system, clocks, GPIO, etc.
        // This assumes that the `split` and initialization methods only borrow from
        // peripherals
        let system_parts = $peripherals.SYSTEM.split();
        let clocks = ClockControl::boot_defaults(&system_parts.clock_control).freeze();

        let io = IO::new($peripherals.GPIO, $peripherals.IO_MUX);

        // Return a tuple or a struct that contains the initialized components
        // and a reference to the peripherals for further use
        Ok(ChipConfig {
            clocks,
            pins: io.pins,
        })
    }};
}

// Define the `ChipConfig`
pub struct ChipConfig {
    // The fields here represent the peripherals that have been initialized
    pub clocks: Clocks<'static>,
    pub pins: Pins,
}

impl ChipConfig {
    pub fn get_i2c(&mut self, periph: Peripherals) -> I2C<'_, impl I2cInstance> {
        I2C::new(
            periph.I2C0,
            &mut self.pins.gpio32,
            &mut self.pins.gpio33,
            100u32.kHz(),
            &self.clocks,
        )
    }

    pub fn get_i2c_with_pins<SDA, SCL>(
        &self,
        periph: Peripherals,
        sda: SDA,
        scl: SCL,
    ) -> I2C<'_, impl I2cInstance>
    where
        SDA: OutputPin + InputPin + Peripheral<P = SDA> + 'static,
        SCL: OutputPin + InputPin + Peripheral<P = SCL> + 'static,
    {
        I2C::new(periph.I2C0, sda, scl, 100u32.kHz(), &self.clocks)
    }

    pub fn get_spi(&mut self, periph: Peripherals) -> Spi<'_, impl SpiInstance, FullDuplexMode> {
        Spi::new(periph.SPI2, 100u32.MHz(), SpiMode::Mode0, &self.clocks).with_pins(
            Some(&mut self.pins.gpio36),
            Some(&mut self.pins.gpio35),
            Some(&mut self.pins.gpio37),
            Some(&mut self.pins.gpio34),
        )
    }

    pub fn get_spi_custom<SCK, MOSI, MISO, CS>(
        &mut self,
        periph: Peripherals,
        sck: SCK,
        mosi: MOSI,
        miso: MISO,
        cs: CS,
        freq: HertzU32,
    ) -> Spi<'_, impl SpiInstance, FullDuplexMode>
    where
        SCK: OutputPin + Peripheral<P = SCK> + 'static,
        MOSI: OutputPin + Peripheral<P = MOSI> + 'static,
        MISO: InputPin + Peripheral<P = MISO> + 'static,
        CS: OutputPin + Peripheral<P = CS> + 'static,
    {
        Spi::new(periph.SPI2, freq, SpiMode::Mode0, &self.clocks).with_pins(
            Some(sck),
            Some(mosi),
            Some(miso),
            Some(cs),
        )
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
