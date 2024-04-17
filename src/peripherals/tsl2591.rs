//! # TSL2591 Light Sensor Module
//!
//! This module offers an interface to the TSL2591 light sensor, enabling the
//! measurement of ambient light intensity in lux. The TSL2591 sensor is capable
//! of high precision light measurement suitable for a variety of lighting
//! conditions.

use embedded_hal::blocking::delay::DelayMs;
use esp_hal::{delay::Delay, i2c::I2C};
use tsl2591_eh_driver::Driver as ExternalTsl2591;

use super::{I2cPeriph, LumiSensor, PeripheralError, UnifiedData};

/// Represents a TSL2591 ambient light sensor.
pub struct Tsl2591Sensor {
    /// The internal TSL2591 driver instance.
    pub inner: ExternalTsl2591<I2C<'static, esp_hal::peripherals::I2C0>>,
    /// Delay provider for timing-sensitive operations.
    pub delay: Delay,
}

impl I2cPeriph for Tsl2591Sensor {
    type Returnable = Self;

    /// Initializes the TSL2591 sensor over the I2C bus.
    ///
    /// This function configures the sensor and enables it for ambient light
    /// measurements.
    ///
    /// # Arguments
    /// * `bus` - The I2C bus instance to communicate with the sensor.
    /// * `delay` - A delay provider for timing-sensitive operations during
    ///   initialization.
    ///
    /// # Returns
    /// A result containing the initialized `Tsl2591Sensor` or an error of type
    /// `PeripheralError` if initialization fails.
    fn create_on_i2c(
        bus: I2C<'static, esp_hal::peripherals::I2C0>,
        delay: Delay,
    ) -> Result<Self::Returnable, PeripheralError> {
        let mut sensor = match ExternalTsl2591::new(bus) {
            Ok(sensor) => sensor,
            Err(_) => return Err(PeripheralError::InitializationFailed),
        };
        match sensor.enable() {
            Ok(_) => {}
            Err(_) => return Err(PeripheralError::InitializationFailed),
        };
        Ok(Tsl2591Sensor {
            inner: sensor,
            delay: delay,
        })
    }
}

impl LumiSensor for Tsl2591Sensor {
    /// Measures the ambient light intensity.
    ///
    /// # Returns
    /// A result containing the light intensity in lux if successful, or an
    /// error of type `PeripheralError` if the measurement fails.
    fn get_lux(&mut self) -> Result<f32, PeripheralError> {
        let (ch_0, ch_1) = self.inner.get_channel_data().unwrap();
        self.delay.delay_ms(500u32);
        match self.inner.calculate_lux(ch_0, ch_1) {
            Ok(light) => Ok(light),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}

impl UnifiedData for Tsl2591Sensor {
    type Output = f32;
    /// Measures the ambient light intensity.
    ///
    /// # Returns
    /// A result containing the light intensity in lux if successful, or an
    /// error of type `PeripheralError` if the measurement fails.
    fn read(&mut self, _delay: Delay) -> Result<f32, PeripheralError> {
        Ok(self.get_lux().unwrap())
    }
}
