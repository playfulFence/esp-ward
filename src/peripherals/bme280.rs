use bme280::{i2c::BME280 as ExternalBME280_i2c, spi::BME280 as ExternalBME280_spi};
// Import the necessary modules from `esp-hal`
use esp_hal::{
    delay::Delay,
    i2c::I2C,
    spi::{master::Spi, FullDuplexMode},
};

use super::{HumiditySensor, I2cPeriph, PeripheralError, PressureSensor, TemperatureSensor};
// TODO: SPI initialization
pub enum Bme280Interface {
    I2C(ExternalBME280_i2c<I2C<'static, esp_hal::peripherals::I2C0>>),
    SPI(ExternalBME280_spi<Spi<'static, esp_hal::peripherals::SPI2, FullDuplexMode>>),
}

pub struct Bme280Sensor {
    pub inner: ExternalBME280_i2c<I2C<'static, esp_hal::peripherals::I2C0>>,
    pub delay: Delay,
}

impl I2cPeriph for Bme280Sensor {
    type Returnable = Self;

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
    fn read_temperature(&mut self) -> Result<f32, PeripheralError> {
        match self.inner.measure(&mut self.delay) {
            Ok(measurement) => Ok(measurement.temperature),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}

impl HumiditySensor for Bme280Sensor {
    fn read_humidity(&mut self) -> Result<f32, PeripheralError> {
        match self.inner.measure(&mut self.delay) {
            Ok(measurement) => Ok(measurement.humidity),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}

impl PressureSensor for Bme280Sensor {
    fn read_pressure(&mut self) -> Result<f32, PeripheralError> {
        match self.inner.measure(&mut self.delay) {
            Ok(measurement) => Ok(measurement.pressure),
            Err(_) => Err(PeripheralError::ReadError),
        }
    }
}
