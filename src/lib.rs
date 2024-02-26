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
        let system = $peripherals.SYSTEM.split();
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

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
        #[cfg(feature = "esp32")]
        return I2C::new(
            periph.I2C0,
            &mut self.pins.gpio32,
            &mut self.pins.gpio33,
            100u32.kHz(),
            &self.clocks,
        );

        #[cfg(feature = "esp32s2")]
        return I2C::new(
            periph.I2C0,
            &mut self.pins.gpio7,
            &mut self.pins.gpio8,
            100u32.kHz(),
            &self.clocks,
        );

        #[cfg(any(
            feature = "esp32s3",
            feature = "esp32c3",
            feature = "esp32c6",
            feature = "esp32h2"
        ))]
        return I2C::new(
            periph.I2C0,
            &mut self.pins.gpio1,
            &mut self.pins.gpio2,
            100u32.kHz(),
            &self.clocks,
        );
    }

    pub fn get_i2c_custom<SDA, SCL>(
        &self,
        periph: Peripherals,
        sda: SDA,
        scl: SCL,
        freq: HertzU32,
    ) -> I2C<'_, impl I2cInstance>
    where
        SDA: OutputPin + InputPin + Peripheral<P = SDA> + 'static,
        SCL: OutputPin + InputPin + Peripheral<P = SCL> + 'static,
    {
        I2C::new(periph.I2C0, sda, scl, freq, &self.clocks)
    }

    pub fn get_spi(&mut self, periph: Peripherals) -> Spi<'_, impl SpiInstance, FullDuplexMode> {
        #[cfg(feature = "esp32")]
        return Spi::new(periph.SPI2, 100u32.MHz(), SpiMode::Mode0, &self.clocks).with_pins(
            Some(&mut self.pins.gpio19),
            Some(&mut self.pins.gpio23),
            Some(&mut self.pins.gpio25),
            Some(&mut self.pins.gpio22),
        );
        #[cfg(feature = "esp32s2")]
        return Spi::new(periph.SPI2, 100u32.MHz(), SpiMode::Mode0, &self.clocks).with_pins(
            Some(&mut self.pins.gpio36),
            Some(&mut self.pins.gpio35),
            Some(&mut self.pins.gpio37),
            Some(&mut self.pins.gpio34),
        );

        #[cfg(any(feature = "esp32c3", feature = "esp32c6",))]
        return Spi::new(periph.SPI2, 100u32.MHz(), SpiMode::Mode0, &self.clocks).with_pins(
            Some(&mut self.pins.gpio6),
            Some(&mut self.pins.gpio7),
            Some(&mut self.pins.gpio2),
            Some(&mut self.pins.gpio10),
        );

        #[cfg(feature = "esp32s3")]
        return Spi::new(periph.SPI2, 100u32.MHz(), SpiMode::Mode0, &self.clocks).with_pins(
            Some(&mut self.pins.gpio12),
            Some(&mut self.pins.gpio13),
            Some(&mut self.pins.gpio11),
            Some(&mut self.pins.gpio10),
        );

        #[cfg(feature = "esp32h2")]
        return Spi::new(periph.SPI2, 100u32.MHz(), SpiMode::Mode0, &self.clocks).with_pins(
            Some(&mut self.pins.gpio1),
            Some(&mut self.pins.gpio3),
            Some(&mut self.pins.gpio2),
            Some(&mut self.pins.gpio11),
        );
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
