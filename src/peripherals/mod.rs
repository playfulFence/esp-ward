mod button;
pub use button::Button;
mod bme280;
pub use bme280::*;
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
use hal::i2c::{Instance as I2cInstance, I2C as I2cHal};

#[derive(Debug)]
pub enum PeripheralError {
    InitializationFailed,
    ReadError,
    WriteError,
    UnsupportedOperation,
}

pub trait Bus {}

struct I2C;
struct SPI;

impl Bus for I2C {}
impl Bus for SPI {}

pub trait Peripheral {
    type Returnable;
    fn create<B: Bus + ?Sized>(bus: B) -> Result<Self::Returnable, PeripheralError>;
}

// Optional trait for peripherals that can be explicitly shutdown or deactivated
pub trait Shutdown {
    fn shutdown(&mut self) -> Result<(), PeripheralError>;
}

// Trait for peripherals capable of reading data (generic)
pub trait Readable {
    type Output;
    fn read(&self) -> Result<Self::Output, PeripheralError>;
}

// Trait for peripherals capable of writing data (generic)
pub trait Writable {
    type Input;
    fn write(&mut self, data: Self::Input) -> Result<(), PeripheralError>;
}

// Specialized trait for temperature sensing peripherals
pub trait TemperatureSensor: Peripheral {
    // Reads the temperature in degrees Celsius
    fn read_temperature(&self) -> Result<f32, PeripheralError>;
}

// Specialized trait for humidity sensing peripherals
pub trait HumiditySensor: Peripheral {
    // Reads the humidity level as a percentage
    fn read_humidity(&self) -> Result<f32, PeripheralError>;
}

// Specialized trait for pressure sensing peripherals
pub trait PressureSensor: Peripheral {
    // Reads the atmospheric pressure in hPa (hectopascals)
    fn read_pressure(&self) -> Result<f32, PeripheralError>;
}
