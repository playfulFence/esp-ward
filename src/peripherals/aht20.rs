use embedded_aht20::{Aht20 as ExternalAht20, DEFAULT_I2C_ADDRESS as DEFAULT};
use esp_hal::{i2c::I2C, Delay};

use super::{HumiditySensor, I2cPeriph, PeripheralError, TemperatureSensor};

pub struct Aht20Sensor {
    pub inner: ExternalAht20<I2C<'static, esp_hal::peripherals::I2C0>, Delay>,
}

impl I2cPeriph for Aht20Sensor {
    type Returnable = Self;

    fn create_on_i2c(
        bus: I2C<'static, esp_hal::peripherals::I2C0>,
        delay: Delay,
    ) -> Result<Self::Returnable, PeripheralError> {
        let sensor = ExternalAht20::new(bus, DEFAULT, delay).unwrap();
        Ok(Aht20Sensor { inner: sensor })
    }
}

impl TemperatureSensor for Aht20Sensor {
    fn read_temperature(&mut self) -> Result<f32, PeripheralError> {
        Ok(self.inner.measure().unwrap().temperature.celcius())
    }
}

impl HumiditySensor for Aht20Sensor {
    fn read_humidity(&mut self) -> Result<f32, PeripheralError> {
        Ok(self.inner.measure().unwrap().relative_humidity)
    }
}
