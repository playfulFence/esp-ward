use embedded_hal::blocking::delay::DelayMs;
use embedded_sgp30::{Sgp30 as ExternalSgp30, I2C_ADDRESS as DEFAULT};
use esp_hal::{delay::Delay, i2c::I2C};

use super::{CO2Sensor, I2cPeriph, PeripheralError, VOCSensor};

pub struct Sgp30Sensor {
    pub inner: ExternalSgp30<I2C<'static, esp_hal::peripherals::I2C0>, Delay>,
    pub delay: Delay,
}

impl I2cPeriph for Sgp30Sensor {
    type Returnable = Self;

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
    fn get_voc(&mut self) -> Result<f32, PeripheralError> {
        self.delay.delay_ms(500u32);
        match self.inner.measure_air_quality() {
            Ok(measurement) => Ok(measurement.tvoc as f32),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}
