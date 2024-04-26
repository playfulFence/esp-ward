//! # BME280 Environmental Sensor Driver
//!
//! This module provides an interface to the BME280 environmental sensor. It
//! allows for reading temperature, humidity, and pressure using the I2C
//! communication protocol.

use bme280::i2c::BME280 as ExternalBME280_i2c;
// use embedded_hal_bus::spi::ExclusiveDevice;
// Import the necessary modules from `esp-hal`
use esp_hal::{delay::Delay, i2c::I2C};

use super::{
    HumiditySensor,
    I2cPeriph,
    PeripheralError,
    PressureSensor,
    TemperatureSensor,
    UnifiedData,
};

/// A sensor instance for the BME280 that provides access to temperature,
/// humidity, and pressure readings.
pub struct Bme280Sensor {
    /// The internal BME280 driver from the `bme280` crate used over I2C.
    pub inner: ExternalBME280_i2c<I2C<'static, esp_hal::peripherals::I2C0>>,
    /// A delay provider for timing-dependent operations.
    pub delay: Delay,
}

impl I2cPeriph for Bme280Sensor {
    type Returnable = Self;
    /// Creates a new instance of the BME280 sensor using the provided I2C bus.
    ///
    /// # Arguments
    /// * `bus` - The I2C bus to use for communication with the sensor.
    /// * `delay` - A delay provider for timing-dependent operations.
    ///
    /// # Returns
    /// Returns an `Ok(Bme280Sensor)` if the sensor is successfully
    /// initialized, or `Err(PeripheralError::InitializationFailed)` if the
    /// sensor cannot be initialized.
    fn create_on_i2c(
        bus: I2C<'static, esp_hal::peripherals::I2C0>,
        mut delay: Delay,
    ) -> Result<Self::Returnable, PeripheralError> {
        let mut sensor = ExternalBME280_i2c::new_primary(bus);
        match sensor.init(&mut delay) {
            Ok(_) => {}
            Err(_) => return Err(PeripheralError::InitializationFailed),
        }
        Ok(Bme280Sensor {
            inner: sensor,
            delay: delay,
        })
    }
}

impl TemperatureSensor for Bme280Sensor {
    /// Reads the current temperature from the BME280 sensor.
    ///
    /// # Returns
    /// Returns an `Ok(f32)` representing the temperature in degrees Celsius if
    /// the read is successful, or `Err(PeripheralError::ReadError)` if the
    /// temperature cannot be read.
    fn get_temperature(&mut self) -> Result<f32, PeripheralError> {
        match self.inner.measure(&mut self.delay) {
            Ok(measurement) => Ok(measurement.temperature),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}

impl HumiditySensor for Bme280Sensor {
    /// Reads the current relative humidity from the BME280 sensor.
    ///
    /// # Returns
    /// Returns an `Ok(f32)` representing the relative humidity in percentage if
    /// the read is successful, or `Err(PeripheralError::ReadError)` if the
    /// humidity cannot be read.
    fn get_humidity(&mut self) -> Result<f32, PeripheralError> {
        match self.inner.measure(&mut self.delay) {
            Ok(measurement) => Ok(measurement.humidity),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}

impl PressureSensor for Bme280Sensor {
    /// Reads the current atmospheric pressure from the BME280 sensor.
    ///
    /// # Returns
    /// Returns an `Ok(f32)` representing the pressure in hPa (hectopascals) if
    /// the read is successful, or `Err(PeripheralError::ReadError)` if the
    /// pressure cannot be read.
    fn get_pressure(&mut self) -> Result<f32, PeripheralError> {
        match self.inner.measure(&mut self.delay) {
            Ok(measurement) => Ok(measurement.pressure),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}

impl UnifiedData for Bme280Sensor {
    type Output = (f32, f32, f32);
    /// Reads the current relative humidity, temperature and pressure from the
    /// BME280 sensor.
    ///
    /// # Returns
    /// Returns an `Ok((f32, f32, f32))` representing the relative
    /// humidity(percentage), temperature(celsious) and pressure(hPa) if the
    /// read is successful, or `Err(PeripheralError::ReadError)` if the data
    /// from sensor cannot be read.
    fn read(&mut self, _delay: Delay) -> Result<Self::Output, PeripheralError> {
        match self.inner.measure(&mut self.delay) {
            Ok(measurement) => Ok((
                measurement.temperature,
                measurement.humidity,
                measurement.pressure,
            )),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}

// TO BE IMPROVED SECTION:
// 1) SPI implementation is blocked due to embedded_hal versions incompatibility
// Skeleton is ready and code should be buldable, but the `OutputPin` type
// incompatibility makes it impossible to implement now and save the whole other
// library pub struct Bme280SensorSpi<CS> {
//     /// The internal BME280 driver from the `bme280` crate used over I2C.
//     pub inner: ExternalBME280_spi<
//         ExclusiveDevice<Spi<'static, esp_hal::peripherals::SPI2,
// FullDuplexMode>, CS, Delay>,     >,
//     /// A delay provider for timing-dependent operations.
//     pub delay: Delay,
// }

// impl<CS: OutputPin<Error = core::convert::Infallible>> Bme280SensorSpi<CS> {
//     pub fn create_on_spi(
//         bus: Spi<'static, esp_hal::peripherals::SPI2, FullDuplexMode>,
//         cs: CS,
//         mut delay: Delay,
//     ) -> Self {
//         let formatted_bus = ExclusiveDevice::new(bus, cs, delay);
//         let mut sensor = ExternalBME280_spi::new(formatted_bus).unwrap();
//         match sensor.init(&mut delay) {
//             Ok(_) => {}
//             Err(_) => return Err(PeripheralError::InitializationFailed),
//         }
//         Ok(Bme280SensorSpi {
//             inner: sensor,
//             delay: delay,
//         })
//     }
// }
