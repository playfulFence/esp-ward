// use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::delay::DelayMs;
use esp_hal::{i2c::I2C, Delay};
use tsl2591_eh_driver::Driver as ExternalTsl2591;

use super::{I2cPeriph, LumiSensor, PeripheralError};

pub struct Tsl2591Sensor {
    pub inner: ExternalTsl2591<I2C<'static, esp_hal::peripherals::I2C0>>,
    pub delay: Delay,
}

impl I2cPeriph for Tsl2591Sensor {
    type Returnable = Self;

    fn create_on_i2c(
        bus: I2C<'static, esp_hal::peripherals::I2C0>,
        delay: Delay,
    ) -> Result<Self::Returnable, PeripheralError> {
        let mut sensor = ExternalTsl2591::new(bus).unwrap();
        sensor.enable().unwrap();
        Ok(Tsl2591Sensor {
            inner: sensor,
            delay: delay,
        })
    }
}

impl LumiSensor for Tsl2591Sensor {
    fn get_lux(&mut self) -> Result<f32, PeripheralError> {
        let (ch_0, ch_1) = self.inner.get_channel_data().unwrap();
        self.delay.delay_ms(500u32);
        Ok(self.inner.calculate_lux(ch_0, ch_1).unwrap())
    }
}
