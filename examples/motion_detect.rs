#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_println::println;
use esp_ward::peripherals::{pir::*, UnifiedData};

#[entry]
fn main() -> ! {
    let peripherals = esp_ward::take_periph!();
    let system = esp_ward::take_system!(peripherals);
    let (_, pins, delay) = esp_ward::initialize_chip!(peripherals, system);

    let mut pir = PIRSensor::create_on_pins(pins.gpio0.into_pull_up_input());

    loop {
        if pir.read(delay).unwrap() {
            println!("Motion detected");
        }
    }
}