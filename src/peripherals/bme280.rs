use bme280::{i2c::BME280 as ExternalBME280_i2c, spi::BME280 as ExternalBME280_spi};
use embedded_hal::blocking::delay::{self, DelayMs};
use embedded_hal_bus::spi::CriticalSectionDevice;
// Import the necessary modules from `esp-hal`
use esp_hal::{
    gpio::{AnyPin, Output},
    i2c::{Instance as I2cInstance, I2C},
    spi::{
        master::{Instance as SpiInstance, Spi},
        FullDuplexMode,
    },
    Delay,
};

use super::{
    HumiditySensor,
    I2cPeriph,
    PeripheralError,
    PressureSensor,
    Readable,
    SpiPeriph,
    TemperatureSensor,
};
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
        sensor.init(&mut delay);
        Ok(Bme280Sensor {
            inner: sensor,
            delay: delay,
        })
    }
}

// impl Readable for Bme280Sensor {
//     type Output = ();
//     fn read(&self, delay: Delay) -> Result<Self::Output, PeripheralError> {}
// }

impl TemperatureSensor for Bme280Sensor {
    fn read_temperature(&mut self) -> Result<f32, PeripheralError> {
        Ok(self.inner.measure(&mut self.delay).unwrap().temperature)
    }
}

impl HumiditySensor for Bme280Sensor {
    fn read_humidity(&mut self) -> Result<f32, PeripheralError> {
        Ok(self.inner.measure(&mut self.delay).unwrap().humidity)
    }
}

impl PressureSensor for Bme280Sensor {
    fn read_pressure(&mut self) -> Result<f32, PeripheralError> {
        Ok(self.inner.measure(&mut self.delay).unwrap().pressure)
    }
}

// impl SpiPeriph for Bme280Sensor {
//     type Returnable = Self;

//     fn create_on_spi(
//         bus: Spi<'static, esp_hal::peripherals::SPI2, FullDuplexMode>,
//         cs: AnyPin<Output<MODE>>,
//         delay: Delay,
//     ) -> Result<Self::Returnable, PeripheralError> {
//         let shared_bus = CriticalSectionDevice::new(bus, cs, delay);
//         let sensor = ExternalBME280_spi::new(shared_bus).unwrap();
//         Ok(Bme280Sensor {
//             inner: Bme280Interface::SPI(sensor),
//         })
//     }
// }
