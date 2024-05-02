#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_ward::display::max7219::*;

#[entry]
fn main() -> ! {
    // Max7219 display requires allocator
    esp_ward::prepare_alloc!();
    let peripherals = esp_ward::take_periph!();
    let system = esp_ward::take_system!(peripherals);
    let (_clocks, pins, delay) = esp_ward::init_chip!(peripherals, system);

    let mut display = Max7219Display::create_on_pins(
        // Data pin
        pins.gpio17.into_push_pull_output(),
        // Cs pin
        pins.gpio16.into_push_pull_output(),
        // Clk pin
        pins.gpio4.into_push_pull_output(),
        // Amount of displays in chain
        7,
        delay,
    );

    display.write_str_looping("Hello, VUT FIT!");

    loop {}
}
