mod button;
pub use button::Button;
pub mod aht20;
pub mod bme280;
use embedded_hal::blocking::delay;
// Import the necessary modules from `esp-hal`
use esp_hal::{
    gpio::{AnyPin, Output},
    i2c::{Instance as I2cInstance, I2C},
    spi::{
        master::{Instance as SpiInstance, Spi},
        FullDuplexMode,
    },
    Delay,
};
#[derive(Debug)]
pub enum PeripheralError {
    InitializationFailed,
    ReadError,
    WriteError,
    UnsupportedBus,
}

pub trait I2cPeriph {
    type Returnable;
    fn create_on_i2c(
        bus: I2C<'static, esp_hal::peripherals::I2C0>,
        delay: Delay,
    ) -> Result<Self::Returnable, PeripheralError>;
}

pub trait SpiPeriph {
    type Returnable;
    fn create_on_spi(
        bus: Spi<'static, esp_hal::peripherals::SPI2, FullDuplexMode>,
        // cs: AnyPin<Output<MODE>>,
        // delay: Delay,
    ) -> Result<Self::Returnable, PeripheralError>;
}

// Optional trait for peripherals that can be explicitly shutdown or deactivated
pub trait Shutdown {
    fn shutdown(&mut self) -> Result<(), PeripheralError>;
}

// Trait for peripherals capable of reading data (generic)
pub trait Readable {
    type Output;
    fn read(&self, delay: Delay) -> Result<Self::Output, PeripheralError>;
}

// Trait for peripherals capable of writing data (generic)
pub trait Writable {
    type Input;
    fn write(&mut self, data: Self::Input) -> Result<(), PeripheralError>;
}

// Specialized trait for temperature sensing peripherals
pub trait TemperatureSensor {
    // Reads the temperature in degrees Celsius
    fn read_temperature(&mut self) -> Result<f32, PeripheralError>;
}

// Specialized trait for humidity sensing peripherals
pub trait HumiditySensor {
    // Reads the humidity level as a percentage
    fn read_humidity(&mut self) -> Result<f32, PeripheralError>;
}

// Specialized trait for pressure sensing peripherals
pub trait PressureSensor {
    // Reads the atmospheric pressure in hPa (hectopascals)
    fn read_pressure(&mut self) -> Result<f32, PeripheralError>;
}
