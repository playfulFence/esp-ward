#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_println::println;
use esp_ward::display::max7219::*;

#[entry]
fn main() -> ! {
    // Max7219 display requires allocator
    esp_ward::prepare_alloc!();
    let peripherals = esp_ward::take_periph!();
    let system = esp_ward::take_system!(peripherals);
    let (_clocks, pins, delay) = esp_ward::initialize_chip!(peripherals, system);

    let mut display = Max7219Display::create_on_pins(
        pins.gpio2.into_push_pull_output(),
        pins.gpio3.into_push_pull_output(),
        pins.gpio4.into_push_pull_output(),
        4,
        delay,
    );

    display.write_str_looping("Hello, BUT FIT!");

    loop {}
}

