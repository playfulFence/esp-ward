#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_println::println;
use esp_ward::peripherals::{
    aht20::*,
    HumiditySensor,
    I2cPeriph,
    PressureSensor,
    TemperatureSensor,
};

#[entry]
fn main() -> ! {
    let peripherals = esp_ward::take_periph!();
    let system = esp_ward::take_system!(peripherals);
    let (clocks, pins, mut delay) = esp_ward::initialize_chip!(peripherals, system);

    let bus = esp_ward::init_i2c_default!(peripherals, pins, clocks);

    let mut sensor = Aht20Sensor::create_on_i2c(bus, delay).unwrap();

    loop {
        println!(
            "Temperature: {}\nHumidity: {}\n\n",
            sensor.get_temperature().unwrap(),
            sensor.get_humidity().unwrap(),
        );

        esp_ward::wait!(delay, 3000);
    }
}
