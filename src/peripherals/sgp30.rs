//! # SGP30 Sensor Module
//!
//! Provides an interface to the SGP30 sensor for air quality measurement,
//! including CO2 and VOC levels. This module abstracts over the
//! `embedded_sgp30` crate to provide a simpler interface for initializing
//! the sensor and reading the air quality metrics.

use embedded_hal::blocking::delay::DelayMs;
use embedded_sgp30::{Sgp30 as ExternalSgp30, I2C_ADDRESS as DEFAULT};
use esp_hal::{delay::Delay, i2c::I2C};

use super::{CO2Sensor, I2cPeriph, PeripheralError, UnifiedData, VOCSensor};

/// Represents an SGP30 air quality sensor.
pub struct Sgp30Sensor {
    /// The internal SGP30 sensor instance.
    pub inner: ExternalSgp30<I2C<'static, esp_hal::peripherals::I2C0>, Delay>,
    /// Delay provider for timing-sensitive operations.
    pub delay: Delay,
}

impl I2cPeriph for Sgp30Sensor {
    type Returnable = Self;

    /// Creates and initializes an SGP30 sensor over the I2C bus.
    ///
    /// Initializes the sensor and starts the air quality measurement process.
    ///
    /// # Arguments
    /// * `bus` - The I2C bus instance to communicate with the sensor.
    /// * `delay` - A delay provider for timing-sensitive operations during
    ///   initialization.
    ///
    /// # Returns
    /// A result containing the initialized `Sgp30Sensor` or an error of type
    /// `PeripheralError` if initialization fails.
    fn create_on_i2c(
        bus: I2C<'static, esp_hal::peripherals::I2C0>,
        delay: Delay,
    ) -> Result<Self::Returnable, PeripheralError> {
        let mut sensor = match ExternalSgp30::new(bus, DEFAULT, delay) {
            Ok(sensor) => sensor,
            Err(_) => return Err(PeripheralError::InitializationFailed),
        };
        match sensor.initialize_air_quality_measure() {
            Ok(_) => {}
            Err(_) => return Err(PeripheralError::InitializationFailed),
        }
        Ok(Sgp30Sensor {
            inner: sensor,
            delay: delay,
        })
    }
}

impl CO2Sensor for Sgp30Sensor {
    /// Measures the CO2 concentration in the air.
    ///
    /// # Returns
    /// A result containing the CO2 concentration in ppm (parts per million) as
    /// `Ok(f32)` if successful, or an error of type `PeripheralError` if the
    /// measurement fails.
    fn get_co2(&mut self) -> Result<f32, PeripheralError> {
        //
        self.delay.delay_ms(500u32);
        match self.inner.measure_air_quality() {
            Ok(measurement) => Ok(measurement.co2 as f32),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}

impl VOCSensor for Sgp30Sensor {
    /// Measures the VOC in the air.
    ///
    /// # Returns
    /// A result containing the VOC as `Ok(f32)` if successful, or an error of
    /// type `PeripheralError` if the measurement fails.
    fn get_voc(&mut self) -> Result<f32, PeripheralError> {
        self.delay.delay_ms(500u32);
        match self.inner.measure_air_quality() {
            Ok(measurement) => Ok(measurement.tvoc as f32),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}

impl UnifiedData for Sgp30Sensor {
    type Output = (f32, f32);
    /// Reads the CO2 concentration in the air and VOC from the
    /// SGP30 sensor.
    ///
    /// # Returns
    /// Returns an `Ok((f32, f32))` representing the relative
    /// CO2 concentration(ppm) and VOC in the air if the
    /// read is successful, or `Err(PeripheralError::ReadError)` if the data
    /// from sensor cannot be read.
    fn read(&mut self, _delay: Delay) -> Result<Self::Output, PeripheralError> {
        self.delay.delay_ms(500u32);
        match self.inner.measure_air_quality() {
            Ok(measurement) => Ok((measurement.co2 as f32, measurement.tvoc as f32)),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}
