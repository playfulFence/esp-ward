//! # AHT20 Sensor Interface
//!
//! This module provides an interface for the AHT20 temperature and humidity
//! sensor. It offers methods to create an instance of the sensor, read
//! temperature, and read humidity data.

use embedded_aht20::{Aht20 as ExternalAht20, DEFAULT_I2C_ADDRESS as DEFAULT};
use esp_hal::{delay::Delay, i2c::I2C};

use super::{HumiditySensor, I2cPeriph, PeripheralError, TemperatureSensor};

/// A sensor instance for the AHT20
pub struct Aht20Sensor {
    /// The internal AHT20 driver from the `embedded_aht20` crate.
    pub inner: ExternalAht20<I2C<'static, esp_hal::peripherals::I2C0>, Delay>,
}

impl I2cPeriph for Aht20Sensor {
    type Returnable = Self;

    /// Creates a new instance of the AHT20 sensor using the provided I2C bus
    /// and delay provider.
    ///
    /// # Arguments
    /// * `bus` - The I2C bus to use for communication.
    /// * `delay` - A delay provider for timing-dependent operations.
    ///
    /// # Returns
    /// Returns an `Ok(Aht20Sensor)` if the sensor is successfully initialized,
    /// or `Err(PeripheralError::InitializationFailed)` if the sensor cannot
    /// be initialized.
    fn create_on_i2c(
        bus: I2C<'static, esp_hal::peripherals::I2C0>,
        delay: Delay,
    ) -> Result<Self::Returnable, PeripheralError> {
        let sensor = match ExternalAht20::new(bus, DEFAULT, delay) {
            Ok(inst) => inst,
            Err(_) => return Err(PeripheralError::InitializationFailed),
        };
        Ok(Aht20Sensor { inner: sensor })
    }
}

impl TemperatureSensor for Aht20Sensor {
    /// Reads the current temperature from the AHT20 sensor.
    ///
    /// # Returns
    /// Returns an `Ok(f32)` representing the temperature in Celsius if the read
    /// is successful, or `Err(PeripheralError::ReadError)` if the
    /// temperature cannot be read.
    fn get_temperature(&mut self) -> Result<f32, PeripheralError> {
        match self.inner.measure() {
            Ok(measurement) => Ok(measurement.temperature.celcius()),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}

impl HumiditySensor for Aht20Sensor {
    /// Reads the current relative humidity from the AHT20 sensor.
    ///
    /// # Returns
    /// Returns an `Ok(f32)` representing the relative humidity(percentage) if
    /// the read is successful, or `Err(PeripheralError::ReadError)` if the
    /// humidity cannot be read.
    fn get_humidity(&mut self) -> Result<f32, PeripheralError> {
        match self.inner.measure() {
            Ok(measurement) => Ok(measurement.relative_humidity),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}
