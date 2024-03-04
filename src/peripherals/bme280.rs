// Import the necessary modules from `esp-hal`
use bme280::{i2c::BME280 as ExternalBME280_i2c, spi::BME280 as ExternalBME280_spi};
use embedded_hal::blocking::delay;
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
use hal::{
    i2c::{Instance as I2cInstance, I2C},
    spi::{
        master::{Instance as SpiInstance, Spi},
        FullDuplexMode,
    },
    Delay,
};

use super::{Bus, HumiditySensor, Peripheral, PeripheralError, PressureSensor, TemperatureSensor};

pub enum Bme280Interface {
    I2C(ExternalBME280_i2c<I2C<'static, I2cInstance>>),
    SPI(ExternalBME280_spi<Spi<'s, SpiInstance, FullDuplexMode>>),
}

pub struct Bme280Sensor {
    inner: Bme280Interface,
}

impl Peripheral for Bme280Sensor {
    type Returnable = Self;

    fn create<B: Bus + ?Sized>(bus: &B) -> Result<Self::Returnable, PeripheralError> {
        // Attempt to downcast the bus reference to a specific type
        if let Some(i2c_bus) = bus.downcast_ref::<I2C>() {
            // Initialize the sensor with an I2C interface
            let sensor =
                ExternalBME280_i2c::new_primary(i2c_bus).map_err(|_| PeripheralError::InitError)?;
            Ok(Bme280Sensor {
                inner: Bme280Interface::I2C(sensor),
            })
        } else if let Some(spi_bus) = bus.downcast_ref::<Spi>() {
            // Initialize the sensor with an SPI interface
            let sensor =
                ExternalBME280_spi::new_primary(spi_bus).map_err(|_| PeripheralError::InitError)?;
            Ok(Bme280Sensor {
                inner: Bme280Interface::SPI(sensor),
            })
        } else {
            Err(PeripheralError::UnsupportedBus)
        }
    }
}
