use embedded_hal::blocking::delay::DelayMs;
use embedded_sgp30::{Sgp30 as ExternalSgp30, I2C_ADDRESS as DEFAULT};
use esp_hal::{i2c::I2C, Delay};

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
        let mut sensor = ExternalSgp30::new(bus, DEFAULT, delay).unwrap();
        sensor.initialize_air_quality_measure().unwrap();
        Ok(Sgp30Sensor {
            inner: sensor,
            delay: delay,
        })
    }
}

impl CO2Sensor for Sgp30Sensor {
    fn get_co2(&mut self) -> Result<f32, PeripheralError> {
        self.delay.delay_ms(500u32);
        Ok(self.inner.measure_air_quality().unwrap().co2 as f32)
    }
}

impl VOCSensor for Sgp30Sensor {
    fn get_voc(&mut self) -> Result<f32, PeripheralError> {
        self.delay.delay_ms(500u32);
        Ok(self.inner.measure_air_quality().unwrap().tvoc as f32)
    }
}
